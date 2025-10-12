use ratatui::{prelude::*};

use crate::ui::keybinds_panel::keybinds_height;
use crate::ui::{
    keybinds_panel::draw_keybinds_panel, queue_preview::draw_queue_preview,
    songs_list::draw_song_list,
};

use super::keybinds_panel::draw_show_keybinds_border;

pub fn draw_file_selector_ui(
    frame: &mut Frame,
    area: Rect,
    songs: &[String],
    queue: &[String],
    selected_index: usize,
    show_keybinds: bool,
) {
    let mut keybinds: Option<[(&str, &str); 8]> = None;

    if show_keybinds {
        keybinds = Some([
            ("j/k", "Navigate"),
            ("Enter", "Play"),
            ("a", "Add"),
            ("p", "Player"),
            ("c", "Queue"),
            ("C", "Play queue"),
            ("q", "Quit"),
            ("?", "Hide this message"),
        ]);
    }

    // Vertical layout: main area + keybinds
    let vertical_chunks = {
        vec![
            Constraint::Min(10),
            Constraint::Length(keybinds_height(keybinds, frame.area().width)),
        ]
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
        draw_keybinds_panel(frame, outer[1], " Keybinds ", &keybinds.unwrap());
    } else {
        draw_show_keybinds_border(frame, outer[1]);
    }
}
