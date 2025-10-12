use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Wrap},
};

pub mod debug;
pub mod file_selector;
pub mod keybinds_panel;
pub mod player;
pub mod queue;
pub mod queue_preview;
pub mod songs_list;
pub mod playing_status;

pub fn draw_home_screen(frame: &mut Frame, area: Rect) {

    let lines: Vec<Line> = vec![
        Line::from(" ██████   ██████                                █████ █████ ███████████"),
        Line::from("▒▒██████ ██████                                ▒▒███ ▒▒███ ▒█▒▒▒███▒▒▒█"),
        Line::from(" ▒███▒█████▒███  █████ ████  █████████  ██████  ▒▒███ ███  ▒   ▒███  ▒ "),
        Line::from(" ▒███▒▒███ ▒███ ▒▒███ ▒███  ▒█▒▒▒▒███  ███▒▒███  ▒▒█████       ▒███    "),
        Line::from(" ▒███ ▒▒▒  ▒███  ▒███ ▒███  ▒   ███▒  ▒███████    ▒▒███        ▒███    "),
        Line::from(" ▒███      ▒███  ▒███ ▒███    ███▒   █▒███▒▒▒      ▒███        ▒███    "),
        Line::from(" █████     █████ ▒▒████████  █████████▒▒██████     █████       █████   "),
        Line::from("▒▒▒▒▒     ▒▒▒▒▒   ▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒     ▒▒▒▒▒       ▒▒▒▒▒    "),
        Line::from(""),
        Line::from(""),
        Line::from("f. Choose song from audio folder"),
        Line::from("?. Show keybinds"),
        Line::from("q. Quit"),
    ];

    let home = Paragraph::new(lines)
        .block(Block::default())
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: false });

    frame.render_widget(home, area);
}
