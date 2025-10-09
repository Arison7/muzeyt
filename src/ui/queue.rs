// ui/queue_view.rs
use ratatui::{prelude::*, Terminal};
use ratatui::backend::CrosstermBackend;
use std::io::Stdout;
use crate::ui::{
    songs_list::draw_song_list,
    keybinds_panel::draw_keybinds_panel,
};

pub fn draw_queue_view(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    queue: &[String],
    selected_index: usize,
    show_keybinds: bool,
) {
    terminal
        .draw(|frame| {
            let area = frame.area();

            // Vertical layout: queue list + keybinds
            let chunks = if show_keybinds {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(5), Constraint::Length(5)])
                    .split(area)
            } else {
                Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Min(5)])
                    .split(area)
            };

            // (a) Queue list (reuse draw_song_list)
            draw_song_list(frame, chunks[0], queue, " Queue ", selected_index);

            // (b) Keybinds
            if show_keybinds {
                let keybinds = [
                    ("j/k", "Navigate"),
                    ("r", "Remove song"),
                    ("n", "Move to top"),
                    ("q", "Exit queue"),
                ];
                draw_keybinds_panel(frame, chunks[1], " Queue Keybinds ", &keybinds);
            }
        })
        .unwrap();
}
