use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Padding},
    Terminal,
};
use ratatui::backend::CrosstermBackend;
use std::io::Stdout;



pub fn draw_song_list(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    songs: &[String],
    selected_index: usize,
) {
    terminal.draw(|frame| {
        let size = frame.area();

        // Build list items
        let items: Vec<ListItem> = songs
            .iter()
            .enumerate()
            .map(|(i, song)| {
                if i == selected_index {
                    // Highlighted (selected) line
                    ListItem::new(song.clone())
                        .style(Style::default().fg(Color::Black).bg(Color::Green))
                } else {
                    // Normal line
                    ListItem::new(song.clone())
                }
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().padding(Padding::uniform(2)).title("Songs").borders(Borders::ALL));

        frame.render_widget(list, size);
    }).unwrap();
}
