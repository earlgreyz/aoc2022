
#[derive(Debug)]
pub enum Token {
    Open,
    Close,
    Separator,
    Value(i32),
}

pub fn run_lexer(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for character in line.chars() {
        match character {
            '[' => tokens.push(Token::Open),
            ']' => tokens.push(Token::Close),
            ',' => tokens.push(Token::Separator),
            _ => {
                let number = character.to_string().parse().unwrap();
                match tokens.last_mut() {
                    Some(Token::Value(value)) => *value = *value * 10 + number,
                    _ => tokens.push(Token::Value(number)),
                };
            }
        }
    }
    tokens
}