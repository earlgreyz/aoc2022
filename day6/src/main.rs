use std::io;
use std::io::BufRead;

mod multiset;

use multiset::MultiSet;

const WINDOW_SIZE: usize = 4;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        let mut window: MultiSet<char> = MultiSet::new();
        let items: Vec<char> = line.chars().collect();

        for i in 0..items.len() {
            window.insert(items[i]);
            if i >= WINDOW_SIZE {
                window.remove(items[i - WINDOW_SIZE])
            }
            if window.len() == WINDOW_SIZE {
                println!("{}", i + 1);
                break
            }
        }
    }
}
