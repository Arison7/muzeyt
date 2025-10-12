pub mod queue;

pub struct ListNavigator<T> {
    pub items: Vec<T>,
    pub selected: usize,
}

impl<T> ListNavigator<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            selected: 0,
        }
    }

    pub fn next(&mut self) {
        if !self.items.is_empty() {
            self.selected = (self.selected + 1) % self.items.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.items.is_empty() {
            if self.selected == 0 {
                self.selected = self.items.len() - 1;
            } else {
                self.selected -= 1;
            }
        }
    }

    pub fn get_selected(&self) -> &T {
        &self.items[self.selected]
    }

}

