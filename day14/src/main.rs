use std::io;
use std::io::BufRead;

use std::ops::{Sub, Add};


#[derive(Clone, Copy, Debug)]
struct Point{ x: usize, y: usize }

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point{ x: self.x - other.x, y: self.y - other.y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point{ x: self.x + other.x, y: self.y + other.y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Air,
    Rock,
    Particle,
}

#[derive(Debug)]
struct Cave {
    start: Point,
    end: Point,
    tiles: Vec<Vec<Tile>>,
}

impl Cave {
    fn at(&self, point: Point) -> Tile {
        self.tiles[point.y][point.x]
    }

    fn place(&mut self, point: Point, tile: Tile) {
        self.tiles[point.y][point.x] = tile;
    }
}

fn read_cave() -> Cave {
    let stdin = io::stdin();
    let rock_lines: Vec<Vec<Point>> = stdin.lock().lines().map(|line| {
        line.unwrap().split(" -> ").map(|point| {
            let (x, y) = point.split_once(",").unwrap();
            Point{ x: x.parse().unwrap(), y: y.parse().unwrap() }
        }).collect()
    }).collect();

    let start_x: usize = rock_lines.iter().map(|line| line.iter().map(|point| point.x).min().unwrap()).min().unwrap();
    let start_y: usize = 0;
    let start = Point{ x: start_x, y: start_y };

    let end_x: usize = rock_lines.iter().map(|line| line.iter().map(|point| point.x).max().unwrap()).max().unwrap();
    let end_y: usize = rock_lines.iter().map(|line| line.iter().map(|point| point.y).max().unwrap()).max().unwrap();
    let end = Point{ x: end_x, y: end_y };

    let mut tiles: Vec<Vec<Tile>> = (start_y..=end_y).map(|_| (start_x..=end_x).map(|_| Tile::Air).collect()).collect();
    for rock_line in rock_lines.iter() {
        for segment in rock_line.windows(2) {
            let segment_start = segment[0] - start;
            let segment_end = segment[1] - start;

            for y in usize::min(segment_start.y, segment_end.y)..=usize::max(segment_start.y, segment_end.y) {
                for x in usize::min(segment_start.x, segment_end.x)..=usize::max(segment_start.x, segment_end.x) {
                    tiles[y][x] = Tile::Rock;
                }
            }
        }
    }

    Cave {
        start: start,
        end: end,
        tiles: tiles,
    }
}

fn part_one(cave: &mut Cave) {
    let mut settled = 0;

    'main: loop {
        let mut particle = Point{ x: 500, y: 0 } - cave.start; // Normalise starting position.
        'particle: loop {
            if particle.y + 1 > cave.end.y {
                break 'main;
            }

            if cave.at(particle + Point{ x: 0, y: 1 }) == Tile::Air {
                particle = particle + Point{ x: 0, y: 1 };
                continue 'particle;
            }

            if particle.x == 0 {
                break 'main;
            }

            if cave.at(particle - Point{ x: 1, y: 0 } + Point{ x: 0, y: 1 }) == Tile::Air {
                particle = particle - Point{ x: 1, y: 0 } + Point{ x: 0, y: 1 };
                continue 'particle;
            }

            if particle.x + 1 > cave.end.x - cave.start.x {
                break 'main;
            }

            if cave.at(particle + Point{ x: 1, y: 1 }) == Tile::Air {
                particle = particle + Point{ x: 1, y: 1 };
                continue 'particle;
            }

            cave.place(particle, Tile::Particle);
            settled += 1;
            break 'particle;
        }
    }

    println!("{}", settled);
}

fn main() {
    let mut cave = read_cave();
    part_one(&mut cave);
}
