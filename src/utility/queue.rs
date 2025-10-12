use std::collections::VecDeque;

pub struct SongQueue {
    current: Option<String>,
    forward: VecDeque<String>,
    back: VecDeque<String>,
    back_limit: usize,
}

impl SongQueue {
    pub fn new(back_limit: usize) -> Self {
        SongQueue {
            current: None,
            forward: VecDeque::new(),
            back: VecDeque::new(),
            back_limit,
        }
    }
    // Returns first song from the top of the queue and updates current
    pub fn get_next_song(&mut self) -> Option<String> {
        if let Some(selector) = self.forward.pop_front() {
            if let Some(current) = &self.current {
                self.back.push_front(current.to_owned());
            }
            if self.back.len() >= self.back_limit {
                self.back.pop_back();
            }
            self.current = Some(selector.clone());
            Some(selector)
        } else {
            None
        }
    }
    // Returns the first song from the previosu Queue sets its as current and adds current song to
    // the next queue
    pub fn get_previous_song(&mut self) -> Option<String> {
        if let Some(selector) = self.back.pop_front() {
            if let Some(current) = &self.current {
                self.forward.push_front(current.to_owned());
            }
            self.current = Some(selector.clone());
            Some(selector)
        } else {
            None
        }
    }

    pub fn queue_file(&mut self, file_name: String) {
        self.forward.push_back(file_name);
    }

    pub fn collect_forward(& self) -> Vec<String> {
        Vec::from_iter(self.forward.clone())
    }

    pub fn remove_forward(&mut self, index: usize) -> Option<String> {
        self.forward.remove(index)
    }

    pub fn push_to_front(&mut self, index: usize) {
        if let Some(item) = self.forward.remove(index) {
            self.forward.push_front(item);
        }
    }
    pub fn set_current(&mut self, current : String) { 
        if let Some(current) = &self.current {
            self.back.push_front(current.to_owned());
        }
        self.current = Some(current);
    }
    // Remove from queue until index is reached 
    pub fn clear_to(&mut self, mut index:  usize) {
        while index > 0 { 
            self.forward.pop_front();
            index -= 1;
        }
    }
}
