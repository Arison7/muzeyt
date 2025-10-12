use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw_now_playing_bar(frame: &mut Frame, area: Rect, song_name: &str) -> Rect {
    // one-line height for the bar
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // bar height
            Constraint::Min(0),    // remaining area
        ])
        .split(area);

    // Create the top bar
    let paragraph = Paragraph::new(format!("Now playing: {}", song_name))
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
        );

    frame.render_widget(paragraph, layout[0]);

    // Return the remaining usable area
    layout[1]
}

