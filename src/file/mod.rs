use std::fs;


pub fn read_files(folder: &str) -> Vec<String> {
    //TODO: This should not just unwrap
    fs::read_dir(folder)
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
        .collect()
}

    /*
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
    
*/

