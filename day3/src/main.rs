use std::io;
use std::io::BufRead;

const LOWERCASE_A_PRIORITY: u8 = 1;
const CAPITAL_A_PRIORITY: u8 = 27;

fn get_priority(item: char) -> u8 {
    let item_ascii = item as u8;

    if item >= 'A' && item <= 'Z' {
        item_ascii - ('A' as u8) + CAPITAL_A_PRIORITY
    } else if item >= 'a' && item <= 'z' {
        item_ascii - ('a' as u8) + LOWERCASE_A_PRIORITY
    } else {
        0
    }
}

fn part_one() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut priorities: i32 = 0;

    while let Some(Ok(line)) = lines.next() {
        let mut first_compartment: [i32; 53] = [0; 53];
        let items = line.chars();
        let items_count = line.len();

        for (index, item) in items.enumerate() {
            let priority = get_priority(item);
            if index < items_count / 2 {
                first_compartment[priority as usize] += 1
            } else if first_compartment[priority as usize] > 0 {
                priorities += priority as i32;
                break;
            }
        }
    }
    println!("{}", priorities)
}

fn main() {
    part_one()
}
