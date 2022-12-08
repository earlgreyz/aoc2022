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

#[allow(dead_code)]
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


struct Score {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl Score {
    fn scenic_score(&self) -> usize {
        self.top * self.bottom * self.left * self.right
    }
}

fn find_scores(trees: &mut Vec<Vec<Tree>>) -> Vec<Vec<Score>> {
    let mut scores: Vec<Vec<Score>> = Vec::new();
    for y in 0..trees.len() {
        scores.push((0..trees[y].len()).map(|_| Score{ top: 0, bottom: 0, left: 0, right: 0}).collect());
    }
    // Find scores from the left.
    for y in 0..trees.len() {
        let mut indexes: [usize; 10] = [0; 10];
        for x in 0..trees[y].len() {
            let height = trees[y][x].height as usize;
            scores[y][x].left = x - indexes[height];
            for i in 0..=height {
                indexes[i] = x;
            }
        }
    }
    // Find scores from the right.
    for y in 0..trees.len() {
        let mut indexes: [usize; 10] = [trees[y].len() - 1; 10];
        for x in (0..trees[y].len()).rev() {
            let height = trees[y][x].height as usize;
            scores[y][x].right = indexes[height] - x;
            for i in 0..=height {
                indexes[i] = x;
            }
        }
    }
    // Find scores from the top.
    for x in 0..trees[0].len() {
        let mut indexes: [usize; 10] = [0; 10];
        for y in 0..trees.len() {
            let height = trees[y][x].height as usize;
            scores[y][x].top = y - indexes[height];
            for i in 0..=height {
                indexes[i] = y;
            }
        }
    }
    // Find scores from the bottom.
    for x in 0..trees[0].len() {
        let mut indexes: [usize; 10] = [trees.len() - 1; 10];
        for y in (0..trees.len()).rev() {
            let height = trees[y][x].height as usize;
            scores[y][x].bottom = indexes[height] - y;
            for i in 0..=height {
                indexes[i] = y;
            }
        }
    }
    scores
}

fn main() {
    let mut trees = load_trees();
    let scores = find_scores(&mut trees);

    let result = scores.iter().map(|row| row.iter().map(|score| score.scenic_score()).max().unwrap()).max().unwrap();
    println!("{}", result);
}