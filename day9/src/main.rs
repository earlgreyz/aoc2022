use std::io;
use std::io::BufRead;
use std::collections::HashSet;
use std::cmp;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn clamp(&self) -> Self {
        let x = if self.x > 1 { 1 } else if self.x < -1 { -1 } else { self.x };
        let y = if self.y > 1 { 1 } else if self.y < -1 { -1 } else { self.y };
        Point{ x: x, y: y }
    }

    fn add_assign(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
    }

    fn sub(&self, other: &Self) -> Self {
        Point{ x: self.x - other.x, y: self.y - other.y }
    }

    fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn parse_move(line: &str) -> (usize, Point) {
    if let Some((direction, count)) = line.split_once(' ') {
        let count: usize = count.parse().unwrap();
        match direction {
            "U" => (count, Point{ x: 0, y: 1 }),
            "R" => (count, Point{ x: 1, y: 0 }),
            "D" => (count, Point{ x: 0, y: -1 }),
            "L" => (count, Point{ x: -1, y: 0 }),
            _ => panic!("invalid direction: {}", direction),
        }
    } else {
        panic!("invalid move: {}", line);
    }
}

fn distance(a: &Point, b: &Point) -> i32 {
    let x_distance = a.x - b.x;
    let y_distance = a.y - b.y;
    cmp::max(x_distance.abs(), y_distance.abs())
}

fn part_one() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut head = Point{ x: 0, y: 0 };
    let mut tail = Point{ x: 0, y: 0 };
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert((0, 0));

    while let Some(Ok(line)) = lines.next() {
        let (count, delta) = parse_move(line.as_ref());
        for _ in 0..count {
            head.add_assign(&delta);
            if distance(&head, &tail) > 1 {
                let tail_delta = head.sub(&tail).clamp();
                tail.add_assign(&tail_delta);
                visited.insert(tail.to_tuple());
            }
        }
    }

    println!("{}", visited.len());
}

fn main() {
    part_one();
}
