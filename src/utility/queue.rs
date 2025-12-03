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
            if self.back.len() > self.back_limit {
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

    pub fn collect_forward(&self) -> Vec<String> {
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
    pub fn set_current(&mut self, current: String) {
        if let Some(current) = &self.current {
            self.back.push_front(current.to_owned());
        }
        self.current = Some(current);
    }
    // Remove from queue until index is reached
    pub fn clear_to(&mut self, mut index: usize) {
        while index > 0 {
            self.forward.pop_front();
            index -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_and_get_next_song() {
        let mut q = SongQueue::new(5);

        q.queue_file("A".into());
        q.queue_file("B".into());
        q.queue_file("C".into());

        assert_eq!(q.get_next_song(), Some("A".into()));
        assert_eq!(q.current, Some("A".into()));

        assert_eq!(q.get_next_song(), Some("B".into()));
        assert_eq!(q.current, Some("B".into()));

        assert_eq!(q.get_next_song(), Some("C".into()));
        assert_eq!(q.current, Some("C".into()));

        assert_eq!(q.get_next_song(), None);
    }

    #[test]
    fn test_back_history_grows_and_next_song_goes_into_back() {
        let mut q = SongQueue::new(5);
        q.queue_file("A".into());
        q.queue_file("B".into());
        q.queue_file("C".into());

        q.get_next_song(); // A
        q.get_next_song(); // B
        q.get_next_song(); // C

        assert_eq!(q.back.len(), 2);
        assert_eq!(q.back[0], "B");
        assert_eq!(q.back[1], "A");
    }

    #[test]
    fn test_get_previous_song() {
        let mut q = SongQueue::new(5);
        q.queue_file("A".into());
        q.queue_file("B".into());

        q.get_next_song(); // A
        q.get_next_song(); // B

        let prev = q.get_previous_song();
        assert_eq!(prev, Some("A".into()));
        assert_eq!(q.current, Some("A".into()));

        // B should have been moved to front of forward
        assert_eq!(q.forward.front(), Some(&"B".into()));
    }

    #[test]
    fn test_back_limit() {
        let mut q = SongQueue::new(2);

        q.queue_file("1".into());
        q.queue_file("2".into());
        q.queue_file("3".into());
        q.queue_file("4".into());

        q.get_next_song(); //1
        q.get_next_song(); //2
        q.get_next_song(); //3

        // back holds ["2", "1"]
        assert_eq!(q.back.len(), 2);

        q.get_next_song(); //4
                           // back now should be ["3", "2"], because limit 2 drops oldest
        assert_eq!(q.back, VecDeque::from(vec!["3".into(), "2".into()]));
    }

    #[test]
    fn test_collect_forward() {
        let mut q = SongQueue::new(5);
        q.queue_file("X".into());
        q.queue_file("Y".into());
        q.queue_file("Z".into());

        assert_eq!(q.collect_forward(), vec!["X", "Y", "Z"]);
    }

    #[test]
    fn test_remove_forward() {
        let mut q = SongQueue::new(5);
        q.queue_file("A".into());
        q.queue_file("B".into());
        q.queue_file("C".into());

        let removed = q.remove_forward(1);
        assert_eq!(removed, Some("B".into()));
        assert_eq!(q.collect_forward(), vec!["A", "C"]);
    }

    #[test]
    fn test_push_to_front() {
        let mut q = SongQueue::new(5);
        q.queue_file("A".into());
        q.queue_file("B".into());
        q.queue_file("C".into());

        q.push_to_front(2);
        assert_eq!(q.collect_forward(), vec!["C", "A", "B"]);
    }

    #[test]
    fn test_set_current_moves_old_current_to_back() {
        let mut q = SongQueue::new(5);

        q.set_current("A".into());
        assert_eq!(q.current, Some("A".into()));
        assert_eq!(q.back.len(), 0);

        q.set_current("B".into());
        assert_eq!(q.current, Some("B".into()));
        assert_eq!(q.back.front(), Some(&"A".into()));
    }

    #[test]
    fn test_clear_to() {
        let mut q = SongQueue::new(5);
        q.queue_file("A".into());
        q.queue_file("B".into());
        q.queue_file("C".into());
        q.queue_file("D".into());

        q.clear_to(2); // removes A, B
        assert_eq!(q.collect_forward(), vec!["C", "D"]);
    }
}
