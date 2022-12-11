use std::collections::{HashMap,VecDeque};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(?P<indent>\s*)(?P<key>[^:]*):\s*(?P<value>.*)$").unwrap();
}

#[derive(Debug)]
pub struct Object {
    value: Option<String>,
    properties: HashMap<String, Box<Object>>,
}

impl Object {
    fn new(value: Option<String>) -> Self {
        Object{ value: value, properties: HashMap::new() }
    }

    pub fn get_value(&self) -> &str {
        self.value.as_ref().unwrap().as_ref()
    }

    pub fn get_property(&self, key: &str) -> &Box<Object> {
        self.properties.get(key).unwrap()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Box<Object>> {
        self.properties.iter()
    }

    pub fn len(&self) -> usize {
        self.properties.len()
    }
}

#[derive(Debug)]
enum Token {
    KeyValue(String, Option<String>),
    Pop,
}

pub struct Parser {
    depths: VecDeque<usize>,
    tokens: Vec<Token>,
}

impl Parser {
    pub fn new() -> Self {
        Parser { depths: VecDeque::new(), tokens: Vec::new() }
    }

    pub fn parse(&mut self, line: &str) {
        if line == "" {
            self.parse_empty(line);
        } else if RE.is_match(line) {
            self.parse_key_value(line);
        } else {
            panic!("Expected line to be empty or match: \"key: value\", got: \"{}\"", line);
        }
    }

    fn parse_empty(&mut self, _: &str) {
        while let Some(_) = self.depths.pop_back() {
            self.tokens.push(Token::Pop);
        }
    }

    fn parse_key_value(&mut self, line: &str) {
        let captures = RE.captures(line).unwrap();
        let depth = captures.name("indent").map_or(0, |indent| indent.as_str().len());
        let key = captures.name("key").unwrap().as_str().to_string();
        let value = captures.name("value").map(|value| value.as_str().to_string()).filter(|value| value.len() > 0);
        
        while let Some(previous_depth) = self.depths.back() {
            if *previous_depth < depth {
                break;
            }
            self.tokens.push(Token::Pop);
            self.depths.pop_back();
        }

        self.tokens.push(Token::KeyValue(key, value));
        self.depths.push_back(depth);
    }

    pub fn build(&self) -> Box<Object> {
        let mut root = Box::new(Object::new(None));
        self.build_object(&mut root, 0);
        root
    }

    fn build_object(&self, parent: &mut Object, start: usize) -> usize {
        let mut index = start;
        while index < self.tokens.len() {
            match &self.tokens[index] {
                Token::Pop => {
                    index += 1;
                    break
                },
                Token::KeyValue(key, value) => {
                    let object_value = value.as_ref().map_or(None, |value| Some(value.to_string()));
                    let mut object = Box::new(Object::new(object_value));
                    index = self.build_object(&mut object, index + 1);
                    parent.properties.insert(key.to_string(), object);
                }
            }
        }
        index
    }
}

