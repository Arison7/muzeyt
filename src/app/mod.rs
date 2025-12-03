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
    path: String,
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
    /// # Creates new instance of App
    ///
    /// # Parameters
    /// - `sink`: rodio sink
    /// - `update_sender`: mspc::Sender resposnbile for sending app Updates
    /// - `path` : path where songs are store
    ///
    /// # Returns
    /// Ok(self) if creation  was successful
    /// Err(std::error:Error) if unsuccessful
    ///
    pub async fn new(
        sink: Arc<Sink>,
        update_sender: mpsc::Sender<AppUpdate>,
        path: String,
        buffer: Arc<Mutex<VecDeque<f32>>>,
        ui_update_sender: mpsc::Sender<UiUpdate>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(App {
            sink,
            status: Status::HomeScreen,
            app_update_sender: update_sender,
            ui_update_sender,
            path,
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
                // TODO : error system
                let songs = read_files(&self.path).expect("failed to read folder");
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
        let total_duration = append_song_from_file(
            (self.path.clone() + "/" + song_name).as_str(),
            &sink,
            &buffer,
        );
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
mod tests {

    use crate::audio_stream;
    use crate::file;

    use super::*;
    const TEST_AUDIO_PATH: &str = "test_audio";

    async fn initialize_app() -> Result<(App, mpsc::Receiver<AppUpdate>), Box<dyn std::error::Error>>
    {
        // sink is initialize here to ensure it's lifetime
        let (sink, _stream) = audio_stream::initialize_stream();

        // Shared buffer (Arc + Mutex so both threads can see it)
        let buffer = Arc::new(Mutex::new(VecDeque::new()));
        // Creates a new pointer to a sink so all threads can access it
        let sink = Arc::new(sink);

        let (update_sender, update_receiver) = tokio::sync::mpsc::channel::<AppUpdate>(32);

        // Channel for updating Ui
        let (ui_update_sender, _): (mpsc::Sender<UiUpdate>, mpsc::Receiver<UiUpdate>) =
            mpsc::channel(32);

        let app = App::new(
            sink,
            update_sender.clone(),
            TEST_AUDIO_PATH.into(),
            buffer,
            ui_update_sender,
        )
        .await?;

        Ok((app, update_receiver))
    }
    #[tokio::test]
    async fn test_status_home_screen() {
        let (app, _) = initialize_app().await.expect("Failed to initialize app");

        // ensure we start with the homeScreen
        assert_eq!(app.status, Status::HomeScreen);
    }
    #[tokio::test]
    async fn test_play_next() {
        let (mut app, mut update_receiver) =
            initialize_app().await.expect("Failed to initialize app");

        let mut queue = SongQueue::new(5);

        let files_names = file::read_files(TEST_AUDIO_PATH).expect("failed to read the test path");

        // We need at least two songs for this test
        assert!(files_names.len() >= 2);

        queue.queue_file(files_names[0].clone());
        queue.queue_file(files_names[1].clone());

        app.song_queue = Some(queue);

        app.app_update_sender
            .send(AppUpdate::PlayNext)
            .await
            .expect("failed to send update");
        if let Some(update) = update_receiver.recv().await {
            app.handle_updates(update).await;
        } else {
            panic!("update not received");
        }
        // assert that there is a song playing
        assert_eq!(app.sink.len(), 1);

        if let Some(mut queue) = app.song_queue {
            // assert that the next song is now first in the queue
            assert_eq!(Some(files_names[1].clone()), queue.get_next_song())
        }
    }
}
