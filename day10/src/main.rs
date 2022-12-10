use std::io;
use std::io::BufRead;

const READINGS: [i32; 7] = [20, 60, 100, 140, 180, 220, std::i32::MAX];

const SPRITE_WIDTH: i32 = 1; // Number of pixel on each side of the sprite centre.
const CRT_WIDTH: i32 = 40;
const LIT_PIXEL: &str = "#";
const DARK_PIXEL: &str = ".";

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_instruction(line: &str) -> Instruction {
    let mut iter = line.split(' ');
    match iter.next().unwrap() {
        "noop" => Instruction::Noop,
        "addx" => {
            let value = iter.next().unwrap().parse().unwrap();
            Instruction::Addx(value)
        }
        unknown => panic!("invalid instruction: {}", unknown)
    }
}

#[allow(dead_code)]
fn part_one() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut cycle = 0;
    let mut x = 1;

    let mut reading = 0;
    let mut signal_strength = 0;

    while let Some(Ok(line)) = lines.next() {
        let instruction = parse_instruction(line.as_ref());
        let (instruction_cycles, x_change) = match instruction {
            Instruction::Noop => (1, 0),
            Instruction::Addx(value) => (2, value),
        };

        if cycle + instruction_cycles >= READINGS[reading] {
            signal_strength +=  READINGS[reading] * x;
            reading += 1;
        }

        cycle += instruction_cycles;
        x += x_change;
    }

    println!("{}", signal_strength);
}

fn is_sprite(position: i32, sprite: i32) -> bool {
    position >= sprite - SPRITE_WIDTH && position <= sprite + SPRITE_WIDTH
}

fn part_two() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut position = -1;
    let mut sprite = 1;

    while let Some(Ok(line)) = lines.next() {
        let instruction = parse_instruction(line.as_ref());
        let (instruction_cycles, x_change) = match instruction {
            Instruction::Noop => (1, 0),
            Instruction::Addx(value) => (2, value),
        };
        for _ in 0..instruction_cycles {
            position = (position + 1) % CRT_WIDTH;
            if position == 0 {
                println!("");
            }
            let pixel = if is_sprite(position, sprite) { LIT_PIXEL } else { DARK_PIXEL };
            print!("{}", pixel);    
        }
        sprite += x_change;
    }
}

fn main() {
    part_two();
}
