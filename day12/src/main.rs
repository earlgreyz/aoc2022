use std::io;
use std::io::BufRead;

use std::collections::VecDeque;

type Position = (usize, usize);
type Map = Vec<Vec<i32>>;

fn read_map() -> (Map, Position, Position) {
    let stdin = io::stdin();

    let mut map = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in stdin.lock().lines().enumerate() {
        let mut row = Vec::new();
        for (x, tile) in line.unwrap().chars().enumerate() {
            let elevation = match tile {
                'S' => {
                    start = (x, y);
                    'a'
                },
                'E' => {
                    end = (x, y);
                    'z'
                }
                elevation => elevation
            };
            row.push(elevation as i32 - 'a' as i32);
        }
        map.push(row);
    } 
    (map, start, end)
}

fn can_move(from: Position, to: Position, map: &Map) -> bool {
    map[from.1][from.0] >= map[to.1][to.0] - 1
}

fn add_neighbours(position: Position, step: usize, map: &Map, queue: &mut VecDeque<(usize, Position)>) {
    let (x, y) = position;
    // Move left
    if x > 0 && can_move(position, (x - 1, y), map) {
        queue.push_back((step, (x - 1, y)));
    }
    // Move right.
    if x < map[y].len() - 1 && can_move(position, (x + 1, y), map) {
        queue.push_back((step, (x + 1, y)));
    }
    // Move up.
    if y > 0 && can_move(position, (x, y - 1), map) {
        queue.push_back((step, (x, y - 1)));
    }
    // Move down.
    if y < map.len() - 1 && can_move(position, (x, y + 1), map) {
        queue.push_back((step, (x, y + 1)));
    }
}

fn part_one(map: &Vec<Vec<i32>>, start: Position, end: Position) {
    let mut queue = VecDeque::new();
    let mut visited = Vec::new();
    for y in 0..map.len() {
        let mut row = Vec::new();
        for _ in 0..map[y].len() {
            row.push(false);
        }
        visited.push(row);
    }

    add_neighbours(start, 1, map, &mut queue);
    visited[start.1][start.0] = true;

    while let Some((step, position)) = queue.pop_front() {
        if position == end {
            println!("{}", step);
            break;
        }
        if !visited[position.1][position.0] {
            add_neighbours(position, step + 1, map, &mut queue);
            visited[position.1][position.0] = true;
        }
    }
}

fn main() {
    let (map, start, end) = read_map();
    part_one(&map, start, end);
}
