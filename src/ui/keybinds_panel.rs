use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};

pub fn draw_keybinds_panel(frame: &mut Frame, area: Rect, title: &str, keybinds: &[(&str, &str)]) {
    // Build one horizontal line of all keybinds
    let mut spans: Vec<Span> = Vec::new();
    for (i, (key, desc)) in keybinds.iter().enumerate() {
        spans.push(Span::styled(
            *key,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::raw(format!(": {}", desc)));

        if i < keybinds.len() - 1 {
            spans.push(Span::raw("  |  ")); // separator
        }
    }

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .padding(Padding::uniform(1)),
        )
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Left);

    frame.render_widget(paragraph, area);
}

pub fn keybinds_height<const N: usize>(
    keybinds: Option<[(&str, &str); N]>,
    area_width: u16,
) -> u16 {
    // total characters (including ": ", "  |  ")
    if let Some(keybinds) = &keybinds {
        let total_len: usize = keybinds
            .iter()
            .enumerate()
            .map(|(i, (key, desc))| {
                let mut len = key.len() + 2 + desc.len(); // ": "
                if i < keybinds.len() - 1 {
                    len += 5; // "  |  "
                }
                len
            })
            .sum();

        // Convert to u16 safely and estimate wrapped lines
        let total_len = total_len.min(u16::MAX as usize) as u16;
        (total_len.div_ceil(area_width).max(1)) + 4 // +2 for borders/padding
    } else {
        1
    }
}
pub fn draw_show_keybinds_border(frame: &mut Frame, area: Rect) {
    let title_line = Line::from(vec![
        Span::raw("-"),
        Span::styled(
            "?",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" : Show keybinds"),
    ]);

    let block = Block::default().borders(Borders::TOP).title(title_line).padding(Padding::bottom(1));

    frame.render_widget(block, area);
}
