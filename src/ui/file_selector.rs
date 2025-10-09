use ratatui::backend::CrosstermBackend;
use ratatui::{prelude::*, Terminal};
use std::io::Stdout;

use crate::ui::{
    keybinds_panel::draw_keybinds_panel, queue_preview::draw_queue_preview,
    songs_list::draw_song_list,
};

pub fn draw_file_selector_ui(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    songs: &[String],
    queue: &[String],
    selected_index: usize,
    show_keybinds: bool,
) {
    terminal
        .draw(|frame| {
            let area = frame.area();

            // Vertical layout: main area + keybinds
            let vertical_chunks = if show_keybinds {
                vec![Constraint::Min(10), Constraint::Length(5)]
            } else {
                vec![Constraint::Min(10)]
            };

            let outer = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vertical_chunks)
                .split(area);

            // Inner layout: songs (2/3) | queue (1/3)
            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
                .split(outer[0]);

            // (a) SONG LIST on the left
            draw_song_list(
                frame,
                horizontal_chunks[0],
                songs,
                " Songs ",
                selected_index,
            );

            // (b) QUEUE PREVIEW on the right
            draw_queue_preview(frame, horizontal_chunks[1], queue, " Queue ", 5);

            // (c) KEYBINDS horizontally at the bottom
            if show_keybinds {
                let keybinds = [
                    ("j/k", "Navigate"),
                    ("Enter", "Play"),
                    ("a", "Add"),
                    ("p", "Play Queue"),
                    ("c", "Queue"),
                    ("q", "Quit"),
                    ("?", "Help"),
                ];
                draw_keybinds_panel(frame, outer[1], " Keybinds ", &keybinds);
            }
        })
        .unwrap();
}
