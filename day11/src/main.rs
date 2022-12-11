use std::io;
use std::io::BufRead;

use std::collections::VecDeque;

use core::str::FromStr;
use std::fmt::Debug;

use std::cmp;

use crate::parser::{Parser, Object};
pub mod parser;

const ROUNDS: usize = 20;
const MORE_ROUNDS: usize = 10000;

#[derive(Debug)]
enum Value {
    Old,
    Const(u64),
}

impl Value {
    fn from_string(text: &str) -> Self {
        if text == "old" {
            Value::Old
        } else {
            Value::Const(text.parse().unwrap())
        }
    }

    fn eval(&self, old: u64) -> u64 {
        match self {
            Value::Old => old,
            Value::Const(value) => *value,
        }
    }
}

#[derive(Debug)]
enum Operation {
    Add(Value, Value),
    Multiply(Value, Value),
}

impl Operation {
    fn from_string(text: &str) -> Self {
        let mut iter = text.split(' ');
        let _new = iter.next();
        let _eq = iter.next();
        let left = Value::from_string(iter.next().unwrap());
        let operation = iter.next().unwrap();
        let right = Value::from_string(iter.next().unwrap());
        match operation {
            "+" => Operation::Add(left, right),
            "*" => Operation::Multiply(left, right),
            _ => panic!("Unknown operation: {}", operation),
        }
    }

    fn eval(&self, old: u64) -> u64 {
        match self {
            Operation::Add(x, y) => x.eval(old) + y.eval(old),
            Operation::Multiply(x, y) => x.eval(old) * y.eval(old),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    modulus: u64,
    pass_true: usize,
    pass_false: usize,
}

fn parse_trailing_number<T: FromStr>(trim: &str, text: &str) -> T where <T as FromStr>::Err: Debug {
    assert_eq!(text[..trim.len()], trim[..]);
    text[trim.len()..].parse::<T>().unwrap()
}

impl Monkey {
    fn from_object(object: &Object) -> Monkey {
        let items_repr = object.get_property("Starting items").get_value();
        let items = items_repr.split(", ").map(|item| item.parse::<u64>().unwrap()).collect();

        let operation_repr = object.get_property("Operation").get_value();
        let operation = Operation::from_string(operation_repr);

        let test_object = object.get_property("Test");
        let modulus = parse_trailing_number::<u64>("divisible by ", test_object.get_value());
        let pass_true_repr = test_object.get_property("If true").get_value();
        let pass_true = parse_trailing_number::<usize>("throw to monkey ", pass_true_repr);
        let pass_false_repr = test_object.get_property("If false").get_value();
        let pass_false = parse_trailing_number::<usize>("throw to monkey ", pass_false_repr);

        Monkey {
            items: items,
            operation: operation,
            modulus: modulus,
            pass_true: pass_true,
            pass_false: pass_false,
        }
    }

    fn default() -> Self {
        Monkey {
            items: VecDeque::new(),
            operation: Operation::Add(Value::Old, Value::Const(0)),
            modulus: 1,
            pass_true: 0,
            pass_false: 0,
        }
    }
}

fn read_monkeys() -> Vec<Monkey> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Run parser for every line in the input.
    let mut parser = Parser::new();
    while let Some(Ok(line)) = lines.next() {
        parser.parse(line.as_ref());
    }

    let root = parser.build();
    let mut monkeys = Vec::new();
    for _ in 0..root.len() {
        monkeys.push(Monkey::default());
    }

    for (name, object) in root.iter() {
        let index = parse_trailing_number::<usize>("Monkey ", name);
        let monkey = Monkey::from_object(object);
        monkeys[index] = monkey;
    }
    monkeys
}

fn monkey_business(inspected: &Vec<usize>) -> usize {
    let (mut first, mut second) = (0, 0);
    for value in inspected {
        second = cmp::max(*value, second);
        if second > first {
            second = first;
            first = *value;
        }
    }
    first * second
}

#[allow(dead_code)]
fn part_one() {
    let mut monkeys = read_monkeys();
    let mut inspected: Vec<usize> = (0..monkeys.len()).map(|_| 0).collect();

    for _ in 0..ROUNDS {
        for i in 0..monkeys.len() {
            inspected[i] += monkeys[i].items.len();
            while let Some(item) = monkeys[i].items.pop_front() {
                let worry = monkeys[i].operation.eval(item) / 3;
                let pass = if worry % monkeys[i].modulus == 0 {
                    monkeys[i].pass_true
                } else {
                    monkeys[i].pass_false
                };
                monkeys[pass].items.push_back(worry);
            }
        }
    }
    let result = monkey_business(&inspected);
    println!("{}", result);
}

fn part_two() {
    let mut monkeys = read_monkeys();
    let mut inspected: Vec<usize> = (0..monkeys.len()).map(|_| 0).collect();

    let mut modulus = monkeys[0].modulus;
    for i in 1..monkeys.len() {
        modulus *= monkeys[i].modulus;
    }

    for _ in 0..MORE_ROUNDS {
        for i in 0..monkeys.len() {
            inspected[i] += monkeys[i].items.len();
            while let Some(item) = monkeys[i].items.pop_front() {
                let worry = monkeys[i].operation.eval(item) % modulus;
                let pass = if worry % monkeys[i].modulus == 0 {
                    monkeys[i].pass_true
                } else {
                    monkeys[i].pass_false
                };
                monkeys[pass].items.push_back(worry);
            }
        }
    }
    let result = monkey_business(&inspected);
    println!("{}", result);
}

fn main() {
    part_two();
}
