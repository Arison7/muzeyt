use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph, Padding, Wrap},
};

pub fn draw_keybinds_panel(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    //NOTE: Anything that can be turned into a str reference
    //TODO: Copy this pattern to the log_debug
    keybinds: &[(impl AsRef<str>, impl AsRef<str>)], // e.g. &[("j / k", "Navigate"), ...]
) {
// Join horizontally, e.g. "j/k Navigate | Enter Play | q Quit"
    let text = keybinds
        .iter()
        .map(|(k, d)| format!("{}: {}", k.as_ref(), d.as_ref()))
        .collect::<Vec<_>>()
        .join("  |  ");

    let paragraph = Paragraph::new(text)
        .block(Block::default().title(title).borders(Borders::ALL).padding(Padding::uniform(1)))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

