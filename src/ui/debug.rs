use ratatui::{
    widgets::{Paragraph,Block,Borders},
    text::Span,
    prelude::*
};


pub fn build_debug_widget(lines: Vec<String>) -> impl ratatui::widgets::Widget {
   let styled_lines: Vec<Line> = lines
        .iter()
        .map(|line| Line::from(Span::styled(line.clone(), Style::default().fg(Color::Yellow))))
        .collect();

    Paragraph::new(styled_lines)
        .block(Block::default().title("Debug").borders(Borders::ALL))
}

