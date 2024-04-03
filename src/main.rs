use std::{io::BufRead, iter::Peekable, str::Chars};

use anyhow::{anyhow, Ok};

#[derive(pest_derive::Parser)]
#[grammar = "src/bash.pest"]
struct Parser;

fn main() -> anyhow::Result<()> {
    let mut input = std::io::stdin().lock();
    loop {
        let mut current_command = String::new();
        let bytes_read = input.read_line(&mut current_command)?;
        if bytes_read == 0 {
            return Err(anyhow!("Unexpected EOF"));
        }
    }
}

fn eval(input: &str) -> (i32, String) {
    let tokens = get_tokens(input).unwrap();

    match tokens.get(0) {
        Some(_) => todo!(),
        None => todo!(),
    }

    return (0, "".into());
}

fn parse_token(chars: &mut Peekable<Chars>) -> Token {
    let mut token = String::from(chars.next().unwrap());
    loop {
        match chars.next() {
            Some(c) => {
                if c.is_whitespace() {
                    break;
                }
                token.push(c);
            }
            None => break,
        }
    }
    return match token.as_str() {
        "echo" => Token::Echo,
        _ => Token::Text(token),
    }
}

fn parse_str(chars: &mut Peekable<Chars>) -> Token {
    let delimiter = chars.next().unwrap();
    let mut token = String::new();
    loop {
        match chars.next() {
            Some(c) => {
                if c == delimiter {
                    break;
                }
                token.push(c);
            }
            None => break,
        }
    }

    
    return Token::Text(token);
}

fn get_tokens(input: &str) -> anyhow::Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    loop {
        match chars.peek() {
            Some('\'' | '\"') => tokens.push(parse_str(&mut chars)),
            Some(c) if !c.is_whitespace() => tokens.push(parse_token(&mut chars)),
            Some(_) => (),
            None => break,
            
        }
    }

    assert_eq!(chars.next(), None, "There was data left in the buffer");
    return Ok(tokens);
}

#[derive(PartialEq, Eq, Debug)]
enum Token {
    Echo,
    Text(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_on_whitespace() {
        let input = "echo hello world";
        let tokens = get_tokens(input).unwrap();
        let expected = vec![
            Token::Echo,
            Token::Text("hello".into()),
            Token::Text("world".into()),
        ];
        assert_eq!(tokens, expected)
    }

    #[test]
    fn parse_strings() {
        let input = "echo \"hello world\"";
        let tokens = get_tokens(input).unwrap();
        let expected = vec![
            Token::Echo,
            Token::Text("hello world".into()),
        ];
        assert_eq!(tokens, expected);

        let input = "echo \'hello world\'";
        let tokens = get_tokens(input).unwrap();
        let expected = vec![
            Token::Echo,
            Token::Text("hello world".into()),
        ];
        assert_eq!(tokens, expected)
    }
}
