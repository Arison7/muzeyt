use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding},
};

pub fn draw_queue_preview(
    frame: &mut Frame,
    area: Rect,
    queue: &[String],
    title: &str,
    max_preview: usize,
) {
    let shown = queue.len().min(max_preview);
    let pending = queue.len().saturating_sub(shown);

    let mut items: Vec<ListItem> = queue
        .iter()
        .take(shown)
        .map(|song| ListItem::new(song.clone()))
        .collect();

    if pending > 0 {
        items.push(
            ListItem::new(format!("+ {pending} more pending..."))
                .style(Style::default().fg(Color::DarkGray).italic()),
        );
    } else if queue.is_empty() {
        items.push(
            ListItem::new("(Queue is empty)")
                .style(Style::default().fg(Color::DarkGray).italic()),
        );
    }

    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL).padding(Padding::uniform(1)));

    frame.render_widget(list, area);
}

