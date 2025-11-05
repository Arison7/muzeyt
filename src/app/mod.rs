use crate::audio_stream::append_song_from_file;
use crate::file::read_files;
use crate::ui::start_ui_loop;
use crate::utility::queue::SongQueue;
use crate::utility::ListNavigator;
use rodio::Sink;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::join;
use tokio::sync::mpsc;

mod events;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Player,
    FileSelector,
    HomeScreen,
    Queue,
}

pub enum AppUpdate {
    PlayNext,
    PlayPrevious,
    Quit,
}
#[derive(Debug, Clone)]
pub struct Song {
    pub duration: Duration,
    pub name: String,
}

#[derive(Debug)]
pub enum UiUpdate {
    Status(Status),
    SelectedIndex(usize),
    CurrentSong(Song),
    Songs(Vec<String>),
    Queue(Vec<String>),
    ShowKeybinds,
    DebugMessage(String),
}

pub struct App {
    pub sink: Arc<Sink>,
    pub status: Status,
    pub running: bool,
    buffer: Arc<Mutex<VecDeque<f32>>>,
    app_update_sender: mpsc::Sender<AppUpdate>,
    ui_update_sender: mpsc::Sender<UiUpdate>,
    navigator: Option<ListNavigator<String>>,
    song_queue: Option<SongQueue>,
    song_duration: Duration,
    watcher_handle: Option<tokio::task::JoinHandle<()>>,
    previous_status: Option<Status>,
}

impl App {
    pub async fn new(
        sink: Sink,
        update_sender: mpsc::Sender<AppUpdate>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Shared buffer (Arc + Mutex so both threads can see it)
        let buffer = Arc::new(Mutex::new(VecDeque::new()));
        // Creates a new pointer to a sink so all threads can access it
        let sink = Arc::new(sink);
        // Channel for updating Ui
        let (ui_update_sender, ui_update_receiver) = mpsc::channel(32);

        // Starts the main ui loop
        // has to run before the initialization of the app so that app can own its variables
        start_ui_loop(&buffer, &sink, ui_update_receiver);

        Ok(App {
            sink,
            status: Status::HomeScreen,
            app_update_sender: update_sender,
            ui_update_sender,
            navigator: None,
            running: true,
            buffer,
            song_duration: Duration::ZERO,
            watcher_handle: None,
            previous_status: None,
            song_queue: None,
        })
    }
    // Starts threat to watch over the duration of the song to play next one once it's over
    fn watch_for_sink_updates(&mut self) {
        let sink = self.sink.clone();
        let sender = self.app_update_sender.clone();
        let handle = tokio::spawn(async move {
            // Small delay between checks to avoid busy-waiting
            let interval = tokio::time::Duration::from_millis(500);
            // If the current song is over (sink is empty) play the next one
            loop {
                if sink.empty() {
                    sender.send(AppUpdate::PlayNext).await.unwrap();
                    break;
                }
                tokio::time::sleep(interval).await;
            }
        });
        self.watcher_handle = Some(handle);
    }

    pub async fn handle_updates(&mut self, update: AppUpdate) {
        match update {
            AppUpdate::PlayNext => {
                // If there is a queue get a mutable reference and get the next song
                if let Some(song) = self
                    .song_queue
                    .as_mut()
                    .and_then(|queue| queue.get_next_song())
                {
                    self.log_debug(format!("Playing next {}", song)).await;
                    self.play_song(song).await;
                } else {
                    // This is a bit confusing ngl
                    // TODO: Adds some message with it
                    self.log_debug("Song failed").await;
                    self.update_status(Status::FileSelector).await;
                }
            }
            AppUpdate::PlayPrevious => {
                if let Some(song) = self
                    .song_queue
                    .as_mut()
                    .and_then(|queue| queue.get_previous_song())
                {
                    self.play_song(song).await;
                } else {
                    self.log_debug("no songs in the previous queue").await;
                }
            }
            AppUpdate::Quit => {
                self.running = false;
            }
        }
    }
    async fn log_debug(&self, message: impl ToString) {
        let msg = message.to_string();
        let _ = self
            .ui_update_sender
            .send(UiUpdate::DebugMessage(msg))
            .await;
    }

    async fn update_status(&mut self, new_status: Status) {
        // Previous status is only used for queue so we don't want to loop it
        self.previous_status = Some(self.status);
        self.status = new_status;
        self.log_debug(format!("{:?}", new_status)).await;
        match new_status {
            Status::FileSelector => {
                // Read all the files from folder
                let songs = read_files("audio");
                // Update ui with the list of songs
                if let Err(err) = self
                    .ui_update_sender
                    .send(UiUpdate::Songs(songs.clone()))
                    .await
                {
                    self.log_debug(err.to_string()).await;
                }
                // Create new navigator
                self.navigator = Some(ListNavigator::new(songs));
            }
            Status::Queue => {
                let songs: Vec<String> = if let Some(queue) = &mut self.song_queue {
                    let songs = queue.collect_forward();

                    let send_queue = self.ui_update_sender.send(UiUpdate::Queue(songs.clone()));
                    let send_index = self.ui_update_sender.send(UiUpdate::SelectedIndex(0));

                    let (res_queue, res_index) = join!(send_queue, send_index);

                    if let Err(err) = res_queue {
                        self.log_debug(err.to_string()).await;
                    }
                    if let Err(err) = res_index {
                        self.log_debug(err.to_string()).await;
                    }

                    songs
                } else {
                    vec![]
                };
                // New navigator
                self.navigator = Some(ListNavigator::new(songs));
            }
            _ => {}
        }

        let _ = self
            .ui_update_sender
            .send(UiUpdate::Status(new_status))
            .await;
    }
    async fn update_queue(&mut self, songs: Vec<String>, selected_index: usize) {
        let send_queue = self.ui_update_sender.send(UiUpdate::Queue(songs.clone()));
        let send_index = self
            .ui_update_sender
            .send(UiUpdate::SelectedIndex(selected_index));
        let mut navigator = ListNavigator::new(songs);
        navigator.selected = selected_index;
        self.navigator = Some(navigator);

        let (_res_queue, _res_index) = join!(send_queue, send_index);
    }

    async fn play_current_file(&mut self) {
        self.update_status(Status::Player).await;
        if let Some(song_name) = self
            .navigator
            .as_mut()
            .map(|nav| nav.get_selected().to_owned())
        {
            if let Some(queue) = &mut self.song_queue {
                queue.set_current(song_name.clone());
            }
            self.play_song(song_name).await;
        }
    }
    // Play song, by file name
    // NOTE:: I don't like concept of passing everything by name of the file, however for the
    // purpose of this app it will have to safise
    async fn play_song(&mut self, song: String) {
        // clear previous handle
        // This is important since appneding new song with Symphonia takes a moment
        // so there is time where the sink is empty during processing
        // which causes auto skipping
        if let Some(previous_handle) = &self.watcher_handle {
            previous_handle.abort();
        }
        let sink = self.sink.clone();
        // Clears sink, but also pauses it
        sink.clear();
        // Resumes the playing
        sink.play();
        let buffer = self.buffer.clone();
        let mut song_name: &str = &song;
        let total_duration =
            append_song_from_file(("audio/".to_owned() + song_name).as_str(), &sink, &buffer);
        // Remove the extension
        if let Some(pos) = song_name.rfind('.') {
            song_name = &song_name[..pos];
        }
        self.song_duration = total_duration;
        let _ = self
            .ui_update_sender
            .send(UiUpdate::CurrentSong(Song {
                duration: total_duration,
                name: song_name.to_owned(),
            }))
            .await;
        self.watch_for_sink_updates();
    }
}

#[cfg(test)]
mod app_core_tests {
    use super::*;

    #[test]
    // Testing if there are songs to perform tests on
    fn test_songs_present() {}
}
