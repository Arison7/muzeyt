pub mod debug;
pub mod file_selector;
mod home_screen;
mod keybinds_panel;
mod player;
mod playing_status;
mod queue;
mod queue_preview;
mod songs_list;

use crate::app::{Song, Status, UiUpdate};
use debug::draw_debug_panel;
use file_selector::draw_file_selector_ui;
use home_screen::draw_home_screen;
use player::draw_player_ui;
use playing_status::draw_now_playing_bar;
use queue::draw_queue_view;

use rodio::Sink;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;

const MAX_DEBUG_LINES: usize = 100;

pub fn start_ui_loop(
    buffer: &Arc<Mutex<VecDeque<f32>>>,
    sink: &Arc<Sink>,
    mut update_receiver: mpsc::Receiver<UiUpdate>,
) {
    tokio::spawn({
        let buffer = buffer.clone();
        let sink = sink.clone();
        //Initialize the Terminal
        let mut terminal = ratatui::init();

        async move {
            let mut status = Status::HomeScreen;
            let mut songs: Vec<String> = vec![];
            let mut queue: Vec<String> = vec![];
            let mut selected_index = 0;
            let mut current_song = Song {
                name: "No song selected".to_owned(),
                duration: Duration::ZERO,
            };
            let mut show_keybinds = false;
            // Force first draw
            let mut dirty = true;
            let mut debug_messages: VecDeque<String> = VecDeque::new();
            loop {
                while let Ok(update) = update_receiver.try_recv() {
                    match update {
                        UiUpdate::Status(s) => {
                            status = s;
                            dirty = true;
                        }
                        UiUpdate::Songs(list) => {
                            songs = list;
                            dirty = true;
                        }
                        UiUpdate::Queue(list) => {
                            queue = list;
                            dirty = true;
                        }
                        UiUpdate::SelectedIndex(idx) => {
                            selected_index = idx;
                            dirty = true;
                        }
                        UiUpdate::CurrentSong(song) => {
                            current_song = song;
                            dirty = true;
                        }
                        UiUpdate::ShowKeybinds => {
                            show_keybinds = !show_keybinds;
                            dirty = true;
                        }
                        UiUpdate::DebugMessage(message) => {
                            debug_messages.push_front(message);
                            // Incase a lot of messages is being send we don't want to take too
                            // much memory
                            if debug_messages.len() > MAX_DEBUG_LINES {
                                debug_messages.pop_back();
                            }
                            dirty = true;
                        }
                    }
                }
                if dirty || (status == Status::Player) {
                    terminal
                        .draw(|frame| {
                            let mut area = frame.area();

                            // let debug handle its own rendering
                            if let Some(main_area) = draw_debug_panel(frame, area, &debug_messages)
                            {
                                area = main_area; // shrink main area if debug visible
                            }

                            // On every view expect of player display top bar
                            // "now playing" if current song has duration different than 0
                            if status != Status::Player && current_song.duration != Duration::ZERO {
                                area = draw_now_playing_bar(frame, area, &current_song.name);
                            }

                            // --- Draw main content ---
                            match status {
                                Status::HomeScreen => draw_home_screen(frame, area),
                                Status::Player => draw_player_ui(
                                    &buffer,
                                    frame,
                                    area,
                                    current_song.duration,
                                    sink.get_pos(),
                                    current_song.name.clone(),
                                    show_keybinds,
                                ),
                                Status::FileSelector => draw_file_selector_ui(
                                    frame,
                                    area,
                                    &songs,
                                    &queue,
                                    selected_index,
                                    show_keybinds,
                                ),
                                Status::Queue => draw_queue_view(
                                    frame,
                                    area,
                                    &queue,
                                    selected_index,
                                    show_keybinds,
                                ),
                            }
                        })
                        .unwrap();
                    dirty = false;
                }

                tokio::time::sleep(Duration::from_millis(33)).await;
            }
        }
    });
}
