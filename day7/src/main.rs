use std::io;
use std::io::BufRead;

const LIMIT: usize = 100000;
const TOTAL_STORAGE: usize = 70000000;
const REQUIRED_FREE_STORAGE: usize = 30000000;

#[derive(Debug)]
enum Command {
    List,
    MoveOut,
    MoveIn(String),
}

fn is_command(line: &str) -> bool {
    line.starts_with("$")
}

fn parse_command(line: &str) -> Command {
    let mut iter = line.split(' ');
    let prompt = iter.next().unwrap();
    assert_eq!(prompt, "$");

    match (iter.next(), iter.next()) {
        (Some("ls"), None) => Command::List,
        (Some("cd"), Some("..")) => Command::MoveOut,
        (Some("cd"), Some(directory)) => Command::MoveIn(String::from(directory)),
        _ => panic!("invalid command: {}", line)
    }
}

#[derive(Debug)]
enum Listing {
    Directory(String),
    File(usize, String),
}

fn parse_listing(line: &str) -> Listing {
    let mut iter = line.split(' ');
    let size = iter.next().unwrap();
    let name = iter.next().unwrap();
    match size {
        "dir" => Listing::Directory(name.to_string()),
        _ => Listing::File(size.parse().unwrap(), name.to_string()),
    }
}


#[derive(Debug)]
enum Node {
    Directory(String, Vec<Box<Node>>),
    File(String, usize),
}

fn build_directory(directory: String, lines: &mut io::Lines<io::StdinLock<'static>>) -> Box<Node> {
    let mut children: Vec<Box<Node>> = Vec::new();
    
    while let Some(Ok(line)) = lines.next() {
        // Either performing a command or listing files in the current directory.
        if is_command(&line) {
            match parse_command(&line) {
                Command::List => {},
                Command::MoveOut => break,
                Command::MoveIn(directory) => {
                    let child = build_directory(directory, lines);
                    children.push(child);
                }
            }
        } else {
            match parse_listing(&line) {
                Listing::Directory(_) => {},
                Listing::File(size, name) => children.push(Box::new(Node::File(name, size))),
            }
        }
    }

    Box::new(Node::Directory(directory, children))
}

fn get_dir_sizes(node: &Node, sizes: &mut Vec<usize>) -> usize {
    match node {
        Node::Directory(_, children) => {
            let size: usize = children.iter().map(|child| get_dir_sizes(&child, sizes)).sum();
            sizes.push(size);
            size
        },
        Node::File(_, size) => *size
    }
}

#[allow(dead_code)]
fn part_one(root: &Node) {
    let mut sizes: Vec<usize> = Vec::new(); 
    get_dir_sizes(&root, &mut sizes);

    let result: usize = sizes.iter().filter(|&size| *size <= LIMIT).sum();
    println!("{}", result);
}

fn part_two(root: &Node) {
    let mut sizes: Vec<usize> = Vec::new(); 
    let total_size = get_dir_sizes(&root, &mut sizes);
    let min_deleted = total_size + REQUIRED_FREE_STORAGE - TOTAL_STORAGE;
    if let Some(result) = sizes.iter().filter(|&size| *size >= min_deleted).min() {
        println!("{}", result);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    if let Some(Ok(line)) = lines.next() {
        match line.as_ref() {
            "$ cd /" => {
                let root = build_directory("/".to_string(), &mut lines);
                part_two(&root);
            },
            _ => panic!("expected: cd /, got: {}", line),
        }
    }
}
