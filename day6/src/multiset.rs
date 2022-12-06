use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

pub struct MultiSet<T> where T: Hash + Eq {
    items: HashMap<T, usize>,
}

impl <T> MultiSet<T> where T: Hash + Eq {
    pub fn new() -> Self {
        MultiSet { 
            items: HashMap::new(), 
        }
    }

    pub fn insert(&mut self, item: T) {
        match self.items.get_mut(&item) {
            Some(count) => *count += 1,
            None => {
                self.items.insert(item, 1);
            },
        }
    }

    pub fn remove(&mut self, item: T) {
        if let Some(count) = self.items.get(&item) {
            if *count > 1 {
                self.items.insert(item, count - 1);
            } else {
                self.items.remove(&item);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}