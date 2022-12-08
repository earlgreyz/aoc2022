use std::io;
use std::io::BufRead;

use std::cmp;
use std::fmt;

struct Tree {
    height: i32,
    visible: bool,
}

impl fmt::Debug for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let symbol = if self.visible { self.height.to_string() } else { "*".to_string() };
        write!(f, "{}", symbol)
    }
}

fn load_trees() -> Vec<Vec<Tree>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut trees: Vec<Vec<Tree>> = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        trees.push(line.chars().map(|tree| {
            let height = tree.to_string().parse().unwrap();
            Tree { height: height, visible: false } 
        }).collect());
    }
    trees
}

fn traverse(trees: &mut Vec<Vec<Tree>>, start: (usize, usize), end: (usize, usize), delta: (i32, i32)) {
    let mut max_height = -1;
    let (mut x, mut y) = start;
    let (dx, dy) = delta;
    loop {
        trees[y][x].visible |= trees[y][x].height > max_height;
        max_height = cmp::max(max_height, trees[y][x].height);
        if (x, y) == end {
            break;
        }
        let (ux, uy) = (x as i32 + dx, y as i32 + dy);
        (x, y) = (ux as usize, uy as usize);
    }
}

fn find_visible_count(trees: &mut Vec<Vec<Tree>>) -> i32 {
    for y in 0..trees.len() {
        // Traverse from left to right;
        traverse(trees, (0, y), (trees[y].len() - 1, y), (1, 0));
        // Traverse from right to left;
        traverse(trees, (trees[y].len() - 1, y), (0, y), (-1, 0));
    }
    for x in 0..trees[0].len() {
        // Traverse from top to bottom;
        traverse(trees, (x, 0), (x, trees.len() - 1), (0, 1));
        // Traverse from bottom to top;
        traverse(trees, (x, trees.len() - 1), (x, 0), (0, -1));
    }
    trees.iter().map(|row| row.iter().map(|tree| if tree.visible { 1 } else { 0 } ).sum::<i32>()).sum::<i32>()
}

fn main() {
    let mut trees = load_trees();
    let result = find_visible_count(&mut trees);
    println!("{}", result);
}
