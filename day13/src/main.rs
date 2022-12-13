use std::io;
use std::io::BufRead;

pub mod lexer;

use crate::lexer::{run_lexer, Token};

#[derive(Debug)]
enum Packet {
    Const(i32),
    List(Vec<Packet>),
}

fn compare_numbers(left: i32, right: i32) -> i32 {
    left - right
}

fn compare_lists(left: &Vec<Packet>, right: &Vec<Packet>) -> i32 {
    let mut index = 0;
    while index < left.len() && index < right.len() {
        let result = compare_packets(&left[index], &right[index]);
        if result != 0 {
            return result;
        }
        index += 1;
    }
    left.len() as i32 - right.len() as i32
}

fn compare_packets(left: &Packet, right: &Packet) -> i32 {
    match (left, right) {
        (Packet::Const(left_value), Packet::Const(right_value)) => compare_numbers(*left_value, *right_value),
        (Packet::List(left_list), Packet::List(right_list)) => compare_lists(left_list, right_list),
        (Packet::Const(left_value), Packet::List(right_list)) => compare_lists(&vec![Packet::Const(*left_value)], right_list),
        (Packet::List(left_list), Packet::Const(right_value)) => compare_lists(left_list, &vec![Packet::Const(*right_value)]),
    }
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

        if compare_packets(&left, &right) < 0 {
            result += index;
        }
        index += 1;
    }
    
    println!("{}", result);
}

fn main() {
    part_one();
}
