use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding},
    Terminal,
};




pub fn draw_song_list(
    frame: &mut Frame,
    area: Rect,
    songs: &[String],
    title: &str,
    selected_index: usize,
) {
    let items: Vec<ListItem> = songs
        .iter()
        .enumerate()
        .map(|(i, song)| {
            if i == selected_index {
                ListItem::new(song.clone())
                    .style(Style::default().fg(Color::Black).bg(Color::Green))
            } else {
                ListItem::new(song.clone())
            }
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title(title).borders(Borders::ALL).padding(Padding::uniform(2)));

    frame.render_widget(list, area);
}
