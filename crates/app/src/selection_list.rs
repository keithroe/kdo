use std::fmt;

pub struct SelectionList<T> {
    selection: Option<usize>,
    items: Vec<T>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for SelectionList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SelectionList")
            .field("selection", &self.selection)
            .field("items", &self.items)
            .finish()
    }
}

impl<T> SelectionList<T> {
    pub fn selection(&self) -> Option<usize> {
        self.selection
    }

    pub fn items(&self) -> &Vec<T> {
        &self.items
    }

    pub fn with_items(items: Vec<T>) -> SelectionList<T> {
        SelectionList {
            selection: if items.is_empty() { None } else { Some(0) },
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.selection {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selection = Some(i);
    }

    pub fn previous(&mut self) {
        let i = match self.selection {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selection = Some(i);
    }

    pub fn select(&mut self, selection: Option<usize>) {
        if let Some(s) = selection {
            if s < self.items.len() {
                self.selection = selection;
            } else {
                self.selection = None;
            }
        } else {
            self.selection = None;
        }
    }

    pub fn unselect(&mut self) {
        self.selection = None;
    }
}
