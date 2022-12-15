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

struct Beacon {
    position: Point,
}

struct Sensor {
    position: Point,
    radius: i32
}

fn read_sensors() -> (Vec<Sensor>, Vec<Beacon>) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut beacons_uniq = HashSet::new();
    let mut sensors = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        let captures = RE.captures(line.as_ref()).unwrap();

        // Sensor
        let sx = captures.name("sx").unwrap().as_str().parse().unwrap();
        let sy = captures.name("sy").unwrap().as_str().parse().unwrap();
        let sensor_position = Point{ x: sx, y: sy };

        // Beacon
        let bx = captures.name("bx").unwrap().as_str().parse().unwrap();
        let by = captures.name("by").unwrap().as_str().parse().unwrap();
        let beacon_position = Point{ x: bx, y: by };

        beacons_uniq.insert((bx, by));
        sensors.push(Sensor{ position: sensor_position, radius: (sensor_position - beacon_position).length() });
    }

    let beacons: Vec<Beacon> = beacons_uniq.iter().map(|(x, y)| Beacon{ position: Point{ x: *x, y: *y }}).collect();
    (sensors, beacons)
}

fn part_one() {
    let (sensors, beacons) = read_sensors();
    let mut row = IntervalTree::new();

    for sensor in sensors.iter() {
        let vertical = (sensor.position.y - SEARCH_ROW).abs();
        let horizontal = sensor.radius - vertical;
        if horizontal >= 0 {
            let interval = (sensor.position.x - horizontal, sensor.position.x + horizontal);
            row.insert(interval);
        }
    }

    let overlapping_beacons = beacons.iter().filter(|beacon| beacon.position.y == SEARCH_ROW).count();
    println!("{}", row.size() - overlapping_beacons as i32);
}

fn main() {
    part_one();
}
