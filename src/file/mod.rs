use std::fs;
use std::vec;

pub struct SongSelector{
    pub songs : Vec<String>, 
    selected : usize,
}

impl SongSelector {
    pub fn new(folder: &str) -> Self {
        let songs = fs::read_dir(folder)
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.is_file() {
                    path.file_name().map(|n| n.to_string_lossy().to_string())
                } else {
                    None
                }
            })
            .collect();

        Self { songs, selected: 0 }
    }

    pub fn next(&mut self) {
        if !self.songs.is_empty() {
            self.selected = (self.selected + 1) % self.songs.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.songs.is_empty() {
            if self.selected == 0 {
                self.selected = self.songs.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }

    // Simple getter so selected cannot be modified from the outside
    pub fn get_selected(&self ) -> usize {
        self.selected
    }
    pub fn get_song(&self) -> &str{
        self.songs[self.selected].as_str()
    }
}

