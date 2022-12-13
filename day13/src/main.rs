use std::io;
use std::io::BufRead;
use std::cmp::Ordering;

pub mod lexer;

use crate::lexer::{run_lexer, Token};

#[derive(Debug)]
enum Packet {
    Const(i32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Const(left_value), Packet::Const(right_value)) => left_value.cmp(right_value),
            (Packet::List(left_list), Packet::List(right_list)) => compare_lists(left_list, right_list),
            (Packet::Const(left_value), Packet::List(right_list)) => compare_lists(&vec![Packet::Const(*left_value)], right_list),
            (Packet::List(left_list), Packet::Const(right_value)) => compare_lists(left_list, &vec![Packet::Const(*right_value)]),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Packet {}

fn compare_lists(left: &Vec<Packet>, right: &Vec<Packet>) -> Ordering {
    let mut index = 0;
    while index < left.len() && index < right.len() {
        let result = left[index].cmp(&right[index]);
        if result != Ordering::Equal {
            return result;
        }
        index += 1;
    }
    left.len().cmp(&right.len())
}

fn run_parser(tokens: &mut std::slice::Iter<Token>) -> Packet {
    let mut builder = Vec::new();
    while let Some(token) = tokens.next() {
        match token {
            Token::Open => builder.push(run_parser(tokens)),
            Token::Close => break,
            Token::Separator => continue,
            Token::Value(value) => builder.push(Packet::Const(*value)),
        }
    }
    Packet::List(builder)
}

fn parse_packet(packet_str: &str) -> Packet {
    let tokens = run_lexer(packet_str);
    let mut iter = tokens.iter();
    let _open = iter.next();
    run_parser(&mut iter)
}

#[allow(dead_code)]
fn part_one() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut index = 1;
    let mut result = 0;

    while let Some(Ok(first)) = lines.next() {
        if first == "" {
            continue
        }
        let second = lines.next().unwrap().unwrap();

        let left = parse_packet(first.as_ref());
        let right = parse_packet(second.as_ref());

        if left < right {
            result += index;
        }
        index += 1;
    }
    
    println!("{}", result);
}

fn part_two() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_divider = Packet::List(vec![Packet::List(vec![Packet::Const(2)])]);
    let second_divider = Packet::List(vec![Packet::List(vec![Packet::Const(6)])]);

    let mut packets = vec![
        Packet::List(vec![Packet::List(vec![Packet::Const(2)])]),
        Packet::List(vec![Packet::List(vec![Packet::Const(6)])])
    ];

    while let Some(Ok(line)) = lines.next() {
        if line == "" {
            continue
        }
        packets.push(parse_packet(line.as_ref()));
    }
    
    packets.sort();

    let mut result = 0;
    for (index, packet) in packets.iter().enumerate() {
        if packet == &first_divider {
            result = index + 1;
        }
        if packet == &second_divider {
            result *= index + 1;
        }
    }
    println!("{}", result);
}

fn main() {
    part_two();
}
