use ratatui::{
    widgets::{Paragraph,Block,Borders,Wrap},
    text::Span,
    prelude::*
};



pub fn build_debug_widget(lines: Vec<String>, area: Rect) -> Paragraph<'static> {
    // Limit number of lines to the height of the frame
    let max_lines = area.height as usize;
    let visible_lines: Vec<Line> = lines
        .iter()
        .rev() // take last lines
        .take(max_lines)
        .rev() // maintain original order
        .map(|line| Line::from(Span::styled(line.clone(), Style::default().fg(Color::Yellow))))
        .collect();

    Paragraph::new(visible_lines)
        .block(Block::default().title("Debug").borders(Borders::ALL))
        .wrap(Wrap { trim: true }) // enable wrapping
}

