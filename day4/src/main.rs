use std::io;
use std::io::BufRead;

fn parse_interval(interval_repr: &str) -> (i32, i32) {
    if let Some((start, end)) = interval_repr.split_once('-') { 
        (start.parse().unwrap(), end.parse().unwrap())
    } else {
        (0, 0)
    }
}

fn contains(interval_a: &(i32, i32), interval_b: &(i32, i32)) -> bool {
    let (a_start, a_end) = interval_a;
    let (b_start, b_end) = interval_b;
    a_start >= b_start && a_end <= b_end
}

#[allow(dead_code)]
fn part_one() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut overlaping: i32 = 0;

    while let Some(Ok(line)) = lines.next() {
        let (a_repr, b_repr) = if let Some((a_repr, b_repr)) = line.split_once(',') { 
            (a_repr, b_repr) 
        } else {
            panic!("Invalid input")
        };
        let interval_a = parse_interval(&a_repr);
        let interval_b = parse_interval(&b_repr);
        if contains(&interval_a, &interval_b) || contains(&interval_b, &interval_a) {
            overlaping += 1;
        }
    }
    println!("{}", overlaping);
}

fn overlaps(interval_a: &(i32, i32), interval_b: &(i32, i32)) -> bool {
    let (a_start, a_end) = interval_a;
    let (b_start, b_end) = interval_b;
    (a_start >= b_start && a_start <= b_end) || (b_start >= a_start && b_start <= a_end)
}

fn part_two() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut overlaping: i32 = 0;

    while let Some(Ok(line)) = lines.next() {
        let (a_repr, b_repr) = if let Some((a_repr, b_repr)) = line.split_once(',') { 
            (a_repr, b_repr) 
        } else {
            panic!("Invalid input")
        };
        let interval_a = parse_interval(&a_repr);
        let interval_b = parse_interval(&b_repr);
        if overlaps(&interval_a, &interval_b) {
            overlaping += 1;
        }
    }
    println!("{}", overlaping);
}


fn main() {
    part_two()
}
