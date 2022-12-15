use std::collections::BTreeMap;
use std::ops::Bound::{Unbounded, Included};
use std::cmp;

pub type Interval = (i32, i32);

pub struct IntervalTree {
    tree: BTreeMap<i32, i32>,
}

impl IntervalTree {
    pub fn new() -> Self {
        IntervalTree{ tree: BTreeMap::new() }
    }

    pub fn insert(&mut self, interval: Interval) {
        let (start, end) = interval;

        let before = self.tree.range((Unbounded, Included(start - 1))).next_back();
        let after = self.tree.range((Unbounded, Included(end + 1))).next_back();

        let interval_start = match before {
            None => start,
            Some((before_start, before_end)) => 
                if *before_end >= start - 1 {
                    *before_start
                } else {
                    start 
                },
        };

        let interval_end = match after {
            None => end,
            Some((_, after_end)) => cmp::max(*after_end, end),
        };

        let overlapping: Vec<i32> = self.tree.range((Included(interval_start), Included(interval_end))).map(|(k, _)| *k).collect();
        for key in overlapping {
            self.tree.remove(&key);
        };

        self.tree.insert(interval_start, interval_end);
    }

    pub fn size(&self) -> i32 {
        self.tree.iter().map(|(start, end)| end - start + 1).sum()
    }
}