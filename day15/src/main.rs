use std::io;
use std::io::BufRead;

use std::collections::HashSet;
use std::ops::Sub;

use regex::Regex;
use lazy_static::lazy_static;

mod interval_tree;
use crate::interval_tree::IntervalTree;

const SEARCH_ROW: i32 = 2000000;
lazy_static! {
    static ref RE: Regex = Regex::new(r"^Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)$").unwrap();
}

#[derive(Clone, Copy, Debug)]
struct Point{ x: i32, y: i32 }

impl Point {
    fn length(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point{ x: self.x - other.x, y: self.y - other.y }
    }
}

fn main() {
    let stdin = io::stdin();
    let points: Vec<(Point, Point)> = stdin.lock().lines().map(|line| line.unwrap()).map(|line| {
        let captures = RE.captures(line.as_ref()).unwrap();
        // Sensor
        let sx = captures.name("sx").unwrap().as_str().parse().unwrap();
        let sy = captures.name("sy").unwrap().as_str().parse().unwrap();
        // Beacon
        let bx = captures.name("bx").unwrap().as_str().parse().unwrap();
        let by = captures.name("by").unwrap().as_str().parse().unwrap();
        (Point{ x: sx, y: sy }, Point{ x: bx, y: by })
    }).collect();

    let beacons_at_row: HashSet<(i32, i32)> = points.iter().map(|(_, beacon)| (beacon.x, beacon.y)).filter(|(_, y)| *y == SEARCH_ROW).collect();

    let mut tree = IntervalTree::new();
    for (sensor, beacon) in points {
        let distance = sensor - beacon;
        let length = distance.length();

        let vertical = (sensor.y - SEARCH_ROW).abs();
        let horizontal = length - vertical;

        if horizontal >= 0 {
            let interval = (sensor.x - horizontal, sensor.x + horizontal);
            tree.insert(interval);
        }
    }


    println!("{}", tree.size() - beacons_at_row.len() as i32);
}
