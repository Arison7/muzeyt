use super::*;
use crossterm::event::KeyEvent;

// All event logic
impl App {
    // Handle keyEvents
    pub async fn handle_event(&mut self, event: KeyEvent) {
        match self.status {
            Status::Player => {
                match event.code {
                    // Quit
                    crossterm::event::KeyCode::Char('q') => {
                        let _ = self.app_update_sender.send(AppUpdate::Quit).await;
                    }
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
                        if self.sink.get_pos() + Duration::new(5, 0) >= self.song_duration {
                            self.sink.clear();
                        }
                        if let Err(e) = self
                            .sink
                            .try_seek(self.sink.get_pos() + Duration::new(5, 0))
                        {
                            self.log_debug(e.to_string()).await;
                        }
                    }
                    // Go back 5s
                    crossterm::event::KeyCode::Char('h') => {
                        let current = self.sink.get_pos();
                        let five_secs = Duration::new(5, 0);

                        if current < Duration::new(1, 0) {
                            // Remove current watcher
                            if let Some(handle) = &self.watcher_handle {
                                handle.abort();
                            }
                            // update app to play previous song
                            self.app_update_sender
                                .send(AppUpdate::PlayPrevious)
                                .await
                                .unwrap();

                            return;
                        }

                        let new_pos = if current > five_secs {
                            current - five_secs
                        } else {
                            Duration::ZERO
                        };

                        if let Err(e) = self.sink.try_seek(new_pos) {
                            self.log_debug(e.to_string()).await;
                        }
                    }
                    // Play next
                    crossterm::event::KeyCode::Char('n') => {
                        let _ = self.app_update_sender.send(AppUpdate::PlayNext).await;
                    }
                    // Play previous
                    crossterm::event::KeyCode::Char('b') => {
                        let _ = self.app_update_sender.send(AppUpdate::PlayPrevious).await;
                    }
                    crossterm::event::KeyCode::Char('f') => {
                        self.update_status(Status::FileSelector).await;
                    }
                    crossterm::event::KeyCode::Char('c') => {
                        self.update_status(Status::Queue).await;
                    }
                    _ => {}
                }
            }
            Status::HomeScreen => {
                match event.code {
                    // Quit
                    crossterm::event::KeyCode::Char('q') => {
                        let _ = self.app_update_sender.send(AppUpdate::Quit).await;
                    }
                    crossterm::event::KeyCode::Char('f') => {
                        self.update_status(Status::FileSelector).await;
                    }
                    _ => {}
                }
            }
            Status::FileSelector => {
                match event.code {
                    // Quit
                    crossterm::event::KeyCode::Char('q') => {
                        let _ = self.app_update_sender.send(AppUpdate::Quit).await;
                    }
                    // Next file in the folder
                    crossterm::event::KeyCode::Char('j') => {
                        if let Some(navigator) = &mut self.navigator {
                            navigator.next();
                            let _ = self
                                .ui_update_sender
                                .send(UiUpdate::SelectedIndex(navigator.selected))
                                .await;
                        }
                    }
                    // Previous file in the folder
                    crossterm::event::KeyCode::Char('k') => {
                        if let Some(navigator) = &mut self.navigator {
                            navigator.prev();
                            let _ = self
                                .ui_update_sender
                                .send(UiUpdate::SelectedIndex(navigator.selected))
                                .await;
                        }
                    }
                    // Append to Queue
                    crossterm::event::KeyCode::Char('a') => {
                        // Create queue if doesn't exist
                        if let Some(nav) = &self.navigator {
                            let queue = self.song_queue.get_or_insert_with(|| SongQueue::new(5));
                            // queue the file
                            queue.queue_file(nav.get_selected().clone());
                            // Update queue in ui
                            let _ = self
                                .ui_update_sender
                                .send(UiUpdate::Queue(queue.collect_forward()))
                                .await;
                        }
                    }
                    // Play next
                    crossterm::event::KeyCode::Char('n') => {
                        self.sink.clear();
                    }
                    // Show queue
                    crossterm::event::KeyCode::Char('c') => {
                        self.update_status(Status::Queue).await;
                    }
                    // Start play from queue
                    crossterm::event::KeyCode::Char('C') => {
                        let _ = self.app_update_sender.send(AppUpdate::PlayNext).await;
                    }
                    // Show Player
                    crossterm::event::KeyCode::Char('p') => {
                        self.update_status(Status::Player).await;
                    }

                    crossterm::event::KeyCode::Enter => self.play_current_file().await,
                    _ => {}
                }
            }
            Status::Queue => {
                match event.code {
                    // Quit queue change back to the previous status
                    crossterm::event::KeyCode::Char('q') => match self.previous_status {
                        Some(previous_status) if previous_status != Status::Queue => {
                            self.update_status(previous_status).await;
                        }
                        _ => {
                            let _ = self.app_update_sender.send(AppUpdate::Quit).await;
                        }
                    },
                    // remove element from the queue
                    crossterm::event::KeyCode::Char('r') => {
                        if let Some(navigator) = &mut self.navigator {
                            if let Some(queue) = &mut self.song_queue {
                                let i = navigator.selected;
                                queue.remove_forward(i);
                                let songs = queue.collect_forward();
                                // Update the queue using the previous index if possible, otherwise use 0
                                let index = i.saturating_sub(1);
                                self.update_queue(songs, index).await;
                            }
                        }
                    }
                    // move down
                    crossterm::event::KeyCode::Char('j') => {
                        if let Some(navigator) = &mut self.navigator {
                            navigator.next();
                            let _ = self
                                .ui_update_sender
                                .send(UiUpdate::SelectedIndex(navigator.selected))
                                .await;
                        }
                    }
                    // move up
                    crossterm::event::KeyCode::Char('k') => {
                        if let Some(navigator) = &mut self.navigator {
                            navigator.prev();
                            let _ = self
                                .ui_update_sender
                                .send(UiUpdate::SelectedIndex(navigator.selected))
                                .await;
                        }
                    }
                    // move to the top of the queue
                    crossterm::event::KeyCode::Char('n') => {
                        if let (Some(navigator), Some(queue)) =
                            (&self.navigator, &mut self.song_queue)
                        {
                            let i = navigator.selected;
                            queue.push_to_front(i);
                            let songs = queue.collect_forward();
                            self.update_queue(songs, i).await;
                        }
                    }
                    crossterm::event::KeyCode::Char('f') => {
                        self.update_status(Status::FileSelector).await;
                    }
                    crossterm::event::KeyCode::Char('p') => {
                        self.update_status(Status::Player).await;
                    }
                    crossterm::event::KeyCode::Enter => {
                        if let (Some(navigator), Some(queue)) =
                            (&self.navigator, &mut self.song_queue)
                        {
                            let i = navigator.selected;
                            queue.clear_to(i);
                        }
                        let _ = self.app_update_sender.send(AppUpdate::PlayNext).await;
                        self.update_status(Status::Player).await;
                    }
                    _ => {}
                }
            }
        }
        if event.code == crossterm::event::KeyCode::Char('?') {
            let _ = self.ui_update_sender.send(UiUpdate::ShowKeybinds).await;
        }
    }
}
