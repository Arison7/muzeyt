use crate::ui::draw_ui;
use crossterm::event::{self, Event, KeyEvent};
use rodio::{OutputStream, Sink};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::audio_stream::append_song_from_file;

pub struct App {
    pub sink: Arc<Sink>,
    pub stream: OutputStream,
    pub status: String,
    pub running: bool,
    buffer: Arc<Mutex<VecDeque<f32>>>,
    debug_lines: Arc<Mutex<Vec<String>>>
    
}

impl App {
    pub async fn new(sink: Sink, stream: OutputStream) -> Result<Self, Box<dyn std::error::Error>> {
        // Shared buffer (Arc + Mutex so both threads can see it)
        let buffer = Arc::new(Mutex::new(VecDeque::new()));
        // let bars = precompute_bars("audio/song.mp3");
        let total_duration = append_song_from_file("audio/song.mp3", &sink, &buffer);

        let sink = Arc::new(sink);

        let debug_lines : Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

        App::start_ui_loop(&buffer,&sink,total_duration, &debug_lines);

        Ok(App {
            sink,
            stream,
            status: String::new(),
            running: true,
            buffer,
            debug_lines
        })
    }
    fn start_ui_loop(buffer: &Arc<Mutex<VecDeque<f32>>>, sink : &Arc<Sink>, total_duration: Duration, debug_lines : &Arc<Mutex<Vec<String>>>) {
        tokio::spawn({
            let buffer = buffer.clone();
            let sink = sink.clone();
            let debug_lines =  debug_lines.clone();
            //Initialize the Terminal
            let mut terminal = ratatui::init();

            async move {
                loop {
                    let current_progress = sink.get_pos(); // refresh inside loop
                    draw_ui(&buffer, &mut terminal, total_duration, current_progress, &debug_lines);

                    tokio::time::sleep(Duration::from_millis(33)).await;
                }
            }
        });
    }
    pub async fn handle_event(
        &mut self,
        event: KeyEvent,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event.code {
            crossterm::event::KeyCode::Char('q') => self.running = false,
            crossterm::event::KeyCode::Char('p') => {
                if self.sink.is_paused() {
                    self.sink.play();
                } else {
                    self.sink.pause();
                }
            },
            crossterm::event::KeyCode::Char('l') => {
                if let Err(e) = self.sink.try_seek(self.sink.get_pos() + Duration::new(5,0)){ 
                    self.log_debug(e.to_string());

                }
            },
            crossterm::event::KeyCode::Char('h') => {
                self.sink.try_seek(self.sink.get_pos() - Duration::new(5,0));
            },
            _ => {}
        }
        Ok(())
    }
    fn log_debug(&self, message: String) {
        let mut lines = self.debug_lines.lock().unwrap();
        lines.push(message);
    }
}
