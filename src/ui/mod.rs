use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use ratatui::backend::CrosstermBackend;
use std::io::Stdout;

pub mod debug;
pub mod player;
pub mod songs_list;
pub mod file_selector;
pub mod queue_preview;
pub mod keybinds_panel;
pub mod queue;

pub fn draw_home_screen(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) {
    terminal.draw(|frame| {
        let size = frame.area();

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
            .block(Block::default().title("Welcome").borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: false });

        frame.render_widget(home, size);
    }).unwrap();
}
