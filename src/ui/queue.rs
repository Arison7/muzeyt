// ui/queue_view.rs
use super::keybinds_panel::draw_show_keybinds_border;
use crate::ui::keybinds_panel::{draw_keybinds_panel, keybinds_height};
use crate::ui::songs_list::draw_song_list;

use ratatui::prelude::*;

pub fn draw_queue_view(
    frame: &mut Frame,
    area: Rect,
    queue: &[String],
    selected_index: usize,
    show_keybinds: bool,
) {
    let mut keybinds: Option<[(&str, &str); 7]> = None;

    if show_keybinds {
        keybinds = Some([
            ("j/k", "Navigate"),
            ("r", "Remove song"),
            ("n", "Move to top"),
            ("Enter", "Play now"),
            ("f", "View files"),
            ("p", "Player"),
            ("q", "Exit queue"),
        ]);
    }

    // Vertical layout: queue list + keybinds
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),
            Constraint::Length(keybinds_height(keybinds, frame.area().width)),
        ])
        .split(area);

    // (a) Queue list (reuse draw_song_list)
    draw_song_list(frame, chunks[0], queue, " Queue ", selected_index);

    // (b) Keybinds
    if show_keybinds {
        draw_keybinds_panel(frame, chunks[1], " Queue Keybinds ", &keybinds.unwrap());
    } else {
        draw_show_keybinds_border(frame, chunks[1]);
    }
}
