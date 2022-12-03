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

#[allow(dead_code)]
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

fn find_common_items(items: &str, index: i32, common: &mut [i32; 53], expected: i32) {
    for item in items.chars() {
        let priority = get_priority(item) as usize;
        if common[priority] == expected {
            common[priority] = index
        }
    }
}

fn part_two() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut priorities: i32 = 0;
    while let (Some(Ok(first)), Some(Ok(second)), Some(Ok(third))) = (lines.next(), lines.next(), lines.next()) {
        let mut common: [i32; 53] = [0; 53];
        
        find_common_items(&first, 1, &mut common, 0);
        find_common_items(&second, 2, &mut common, 1);
        find_common_items(&third, 3, &mut common, 2);
        
        for index in 1..53 {
            if common[index] == 3 {
                priorities += index as i32;
                break;
            }
        }
    }
    println!("{}", priorities)
}

fn main() {
    part_two()
}
