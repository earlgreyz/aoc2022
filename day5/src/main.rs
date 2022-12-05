use std::io;
use std::io::BufRead;
use std::collections::VecDeque;

fn read_stacks(stdin: &io::Stdin) -> Vec<VecDeque<char>> {
    let mut lines = stdin.lock().lines();
    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    loop {
        let line = if let Some(Ok(line)) = lines.next() { line } else { panic!("missing input") };
        match line.as_ref() {
            "" => break,
            line => {
                let items: Vec<char> = line.chars().collect();
                let items_count = line.len() / 4;
    
                // Add the stacks if they don't exist.
                for _ in stacks.len()..=items_count {
                    stacks.push(VecDeque::new());
                }
    
                // Read line and push items to the stacks.
                for index in 0..=items_count {
                    let item = items[index * 4 + 1];
                    if (item >= 'a' && item <= 'z') || (item >= 'A' && item <= 'Z') {
                        stacks[index].push_back(item);
                    }
                }
            },
        }
    }
    stacks
}

fn parse_move(instruction: &str) -> (usize, usize, usize) {
    let mut iter = instruction.split(' ');
    let _ = iter.next();
    let count: usize = iter.next().unwrap().parse().unwrap();
    let _ = iter.next();
    let from: usize = iter.next().unwrap().parse().unwrap();
    let _ = iter.next();
    let to: usize = iter.next().unwrap().parse().unwrap();
    (count, from, to)
}

#[allow(dead_code)]
fn perform_move_part_one(stacks: &mut Vec<VecDeque<char>>, count: usize, from: usize, to: usize) {
    for _ in 0..count {
        match stacks[from - 1].pop_front() {
            Some(item) => stacks[to - 1].push_front(item),
            None => panic!("invalid move operation"),
        }
    }
}

fn perform_move_part_two(stacks: &mut Vec<VecDeque<char>>, count: usize, from: usize, to: usize) {
    let mut items: Vec<char> = Vec::new();
    for _ in 0..count {
        match stacks[from - 1].pop_front() {
            Some(item) => items.push(item),
            None => panic!("invalid move operation"),
        }
    }
    for item in items.iter().rev() {
        stacks[to - 1].push_front(*item);
    }
}

fn program() {
    let stdin = io::stdin();
    let mut stacks = read_stacks(&stdin);

    let mut lines = stdin.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        let (count, from, to) = parse_move(&line);
        perform_move_part_two(&mut stacks, count, from, to);
    }

    for stack in stacks {
        if let Some(item) = stack.front() {
            print!("{}", item);
        }
    }
}

fn main() {
    program();
}
