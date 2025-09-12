use crate::ui::{compute_bars, draw_visualization};
use crossterm::event::{self, Event, KeyEvent};
use rodio::{OutputStream, Sink};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::audio_stream::append_song_from_file;
use crate::ui::BAR_COUNT;

pub struct App {
    pub sink: Sink,
    pub stream: OutputStream,
    pub status: String,
    pub running: bool,
    buffer: Arc<Mutex<VecDeque<f32>>>,
}

impl App {
    pub async fn new(sink: Sink, stream: OutputStream) -> Result<Self, Box<dyn std::error::Error>> {
        // Shared buffer (Arc + Mutex so both threads can see it)
        let buffer = Arc::new(Mutex::new(VecDeque::new()));
        // let bars = precompute_bars("audio/song.mp3");
        append_song_from_file("audio/song.mp3", &sink, &buffer);

        App::start_ui_loop(&buffer);

        Ok(App {
            sink,
            stream,
            status: String::new(),
            running: true,
            buffer,
        })
    }
    fn start_ui_loop(buffer: &Arc<Mutex<VecDeque<f32>>>) {
        tokio::spawn({
            let buffer = buffer.clone();

            let labels: Vec<String> = (0..BAR_COUNT).map(|i| i.to_string()).collect();

            //Initialize the Terminal
            let mut terminal = ratatui::init();
            async move {
                loop {
                    draw_visualization(&buffer, &mut terminal, &labels);

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
            }
            _ => {}
        }
        Ok(())
    }
}
