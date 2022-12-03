use std::io;
use std::io::BufRead;

fn part_one() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut score: i32 = 0;

    while let Some(Ok(line)) = lines.next() {
        let result = match line.trim().as_ref() {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => 0,
        };
        score += result
    }
    println!("{}", score);
}

fn main() {
    part_one()
}
