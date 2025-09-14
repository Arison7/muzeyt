use crossterm::event::{self, Event, KeyEvent};
use rodio::{OutputStream, Sink};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::watch::{self, Receiver, Sender};

use crate::audio_stream::append_song_from_file;
use crate::file::SongSelector;
use crate::ui::draw_home_screen;
use crate::ui::file_selector::draw_song_list;
use crate::ui::player::draw_player_ui;

#[derive(Debug, Clone, Copy)]
enum Status {
    Player,
    FileSelector,
    HomeScreen,
}

#[derive(Debug, Clone)]
pub struct Song {
    pub duration: Duration,
    pub name: String,
}

pub struct App {
    pub sink: Arc<Sink>,
    pub stream: OutputStream,
    pub status: Status,
    pub running: bool,
    buffer: Arc<Mutex<VecDeque<f32>>>,
    debug_lines: Arc<Mutex<Vec<String>>>,
    status_sender: Sender<Status>,
    selected_song_sender: Sender<usize>,
    current_song_sender: Sender<Song>,
    song_selector: SongSelector,
}

impl App {
    pub async fn new(sink: Sink, stream: OutputStream) -> Result<Self, Box<dyn std::error::Error>> {
        // Shared buffer (Arc + Mutex so both threads can see it)
        let buffer = Arc::new(Mutex::new(VecDeque::new()));
        // Creates a new pointer to a sink so all threads can access it
        let sink = Arc::new(sink);
        // Creates a new innerly mutable pointer to Vector which is gonna store debug lines
        // Not the most efficient way of doing it probably, but this is for debuging so I think
        // it's alright
        // TODO: Change in into the channel
        let debug_lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

        // Updates for ui_loop about the app status
        let (status_sender, status_receiver) = watch::channel(Status::HomeScreen);

        // Updates for ui_lopp about selected song in the file system
        let (selected_song_sender, selected_song_receiver) = watch::channel(0);

        // Provides details of current song to the player
        let (current_song_sender, current_song_receiver): (Sender<Song>, Receiver<Song>) =
            watch::channel(Song {
                name: "unkown".to_string(),
                duration: Duration::ZERO,
            });

        let song_selector = SongSelector::new("audio");

        // Starts the main ui loop
        // has to run before the initialization of the app so that app can own its variables
        // NOTE: I don't like cloing the songs here
        App::start_ui_loop(
            &buffer,
            &sink,
            &debug_lines,
            status_receiver,
            selected_song_receiver,
            current_song_receiver,
            song_selector.songs.clone(),

        );

        Ok(App {
            sink,
            stream,
            status: Status::HomeScreen,
            status_sender,
            selected_song_sender,
            current_song_sender,
            song_selector,
            running: true,
            buffer,
            debug_lines,
        })
    }
    fn start_ui_loop(
        buffer: &Arc<Mutex<VecDeque<f32>>>,
        sink: &Arc<Sink>,
        debug_lines: &Arc<Mutex<Vec<String>>>,
        status_receiver: Receiver<Status>,
        selected_song_receiver: Receiver<usize>,
        current_song_receiver: Receiver<Song>,
        songs: Vec<String>,
    ) {
        tokio::spawn({
            let buffer = buffer.clone();
            let sink = sink.clone();
            let debug_lines = debug_lines.clone();
            //Initialize the Terminal
            let mut terminal = ratatui::init();

            async move {
                loop {
                    // Check current status to determinate the screen to display
                    let status = *status_receiver.borrow();

                    match status {
                        Status::HomeScreen => {
                            draw_home_screen(&mut terminal);
                        }
                        Status::Player => {
                            let current_progress = sink.get_pos(); // refresh inside loop
                            let song  = current_song_receiver.borrow().clone();
                            draw_player_ui(
                                &buffer,
                                &mut terminal,
                                song.duration,
                                current_progress,
                                &debug_lines,
                            );
                        }
                        Status::FileSelector => {
                            let selected_index = *selected_song_receiver.borrow();
                            draw_song_list(&mut terminal, &songs, selected_index);
                        }
                    }

                    tokio::time::sleep(Duration::from_millis(33)).await;
                }
            }
        });
    }
    pub async fn handle_event(& mut self, event: KeyEvent) {
        match self.status {
            Status::Player => {
                match event.code {
                    // Quit
                    crossterm::event::KeyCode::Char('q') => self.running = false,
                    // Pause
                    crossterm::event::KeyCode::Char('p') => {
                        if self.sink.is_paused() {
                            self.sink.play();
                        } else {
                            self.sink.pause();
                        }
                    }
                    // Skip 5s
                    crossterm::event::KeyCode::Char('l') => {
                        if let Err(e) = self
                            .sink
                            .try_seek(self.sink.get_pos() + Duration::new(5, 0))
                        {
                            self.log_debug(e.to_string());
                        }
                    }
                    // Go back 5s
                    crossterm::event::KeyCode::Char('h') => {
                        let current = self.sink.get_pos();
                        let five_secs = Duration::new(5, 0);

                        let new_pos = if current > five_secs {
                            current - five_secs
                        } else {
                            Duration::ZERO
                        };

                        if let Err(e) = self.sink.try_seek(new_pos) {
                            self.log_debug(e.to_string());
                        }
                    }
                    _ => {}
                }
            }
            Status::HomeScreen => {
                match event.code {
                    // Quit
                    crossterm::event::KeyCode::Char('q') => self.running = false,
                    crossterm::event::KeyCode::Char('f') => {
                        self.update_status(Status::FileSelector)
                    }
                    _ => {}
                }
            }
            Status::FileSelector => {
                match event.code {
                    // Quit
                    crossterm::event::KeyCode::Char('q') => self.running = false,
                    // Next file in the folder
                    crossterm::event::KeyCode::Char('j') => self.next_file(),
                    // Previous file in the folder
                    crossterm::event::KeyCode::Char('k') => self.prev_file(),
                    crossterm::event::KeyCode::Enter => self.play_current_song(),
                    _ => {}
                }
            }
        }
    }
    fn log_debug(&self, message: String) {
        let mut lines = self.debug_lines.lock().unwrap();
        lines.push(message);
    }

    fn update_status(&mut self, new_status: Status) {
        self.status = new_status;
        if let Err(e) = self.status_sender.send(new_status) {
            self.log_debug(e.to_string());
        }
    }
    fn next_file(&mut self) {
        self.song_selector.next();
        if let Err(e) = self
            .selected_song_sender
            .send(self.song_selector.get_selected())
        {
            self.log_debug(e.to_string());
        }
    }
    fn prev_file(&mut self) {
        self.song_selector.prev();
        if let Err(e) = self
            .selected_song_sender
            .send(self.song_selector.get_selected())
        {
            self.log_debug(e.to_string());
        }
    }
    fn play_current_song(& mut self) {
        let sink = self.sink.clone();
        let buffer = self.buffer.clone();
        self.update_status(Status::Player);
        let mut song_name = self.song_selector.get_song();
        let total_duration =
            append_song_from_file(("audio/".to_owned() + song_name).as_str(), &sink, &buffer);
        // Remove the extension
        if let Some(pos) = song_name.rfind('.') {
            song_name = &song_name[..pos];
        }
        if let Err(e) = self.current_song_sender.send(Song {
            duration: total_duration,
            name: song_name.to_owned(),
        }) {
            self.log_debug(e.to_string());
        }
    }
}
