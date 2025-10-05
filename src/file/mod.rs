use std::fs;
use std::vec;
use std::collections::VecDeque;


pub struct SongSelector{
    pub songs : Vec<String>, 
    selected : usize,
    current: Option<usize>,
    back: VecDeque<usize>,
    forward: VecDeque<usize>,
    back_limit: usize,
}

impl SongSelector {
    pub fn new(folder: &str, back_limit: usize) -> Self {
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

        Self { songs, selected: 0,
            current: None,
            back: VecDeque::with_capacity(back_limit),
            forward: VecDeque::new(),
            back_limit,
        }
    }

    pub fn next_file(&mut self) {
        if !self.songs.is_empty() {
            self.selected = (self.selected + 1) % self.songs.len();
        }
    }

    pub fn prev_file(&mut self) {
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
    pub fn get_song(&mut self) -> &str{
        self.current = Some(self.selected);
        self.songs[self.selected].as_str()
        
    }


    // Returns first song from the top of the queue and updates current
    pub fn get_next_song(&mut self) -> Option<String> { 
        if let Some(selector) =  self.forward.pop_front() {
            if let Some(current) = self.current {
                self.back.push_front(current);
            }
            if self.back.len() >= self.back_limit {
                self.back.pop_back();
            }
            self.current = Some(selector);
            Some(self.songs[selector].clone())
        }else {
            None
        }
    }
    // Returns the first song from the previosu Queue sets its as current and adds current song to
    // the next queue
    pub fn get_previous_song(&mut self) -> Option<String> {
        if let Some(selector) = self.back.pop_front() {
            self.forward.push_front(self.current?);
            self.current = Some(selector);
            Some(self.songs[selector].clone())
        }else {
            None
        }
    }

    pub fn queue_file(&mut self){
        self.forward.push_back(self.selected);
    }
    

}

