use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::collections::VecDeque;

pub fn draw_debug_panel(
    frame: &mut ratatui::Frame,
    area: Rect,
    debug_lines: &VecDeque<String>,
) -> Option<Rect> {
    if debug_lines.is_empty() {
        return None;
    }

    // compute height dynamically (up to 8)
    let height = debug_lines.len().min(8) as u16;

    // split area into main + debug sections
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height + 2), // +2 for borders
        ])
        .split(area);

    // build visible lines
    let lines: Vec<Line> = debug_lines
        .iter()
        .take(height as usize)
        .rev()
        .map(|line| Line::from(Span::styled(line, Style::default().fg(Color::Yellow))))
        .collect();

    let widget = Paragraph::new(lines)
        .block(Block::default().title("Debug").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

    frame.render_widget(widget, chunks[1]);

    Some(chunks[0]) // return the remaining (main) area
}

