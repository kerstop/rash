use std::{io::BufRead, iter::Peekable, str::Chars};

use anyhow::{anyhow, Ok};

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

fn get_tokens(input: &str) -> anyhow::Result<Vec<Token>> {
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
        return Token::Text(token);
    }
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    loop {
        match chars.peek() {
            Some(c) => {
                if !c.is_whitespace() {
                    tokens.push(parse_token(&mut chars));
                }
            }
            None => {
                tokens.push(Token::EOC);
                break;
            }
        }
    }

    assert_eq!(chars.next(), None, "There was data left in the buffer");
    return Ok(tokens);
}

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    EOC,
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
            Token::Text("echo".into()),
            Token::Text("hello".into()),
            Token::Text("world".into()),
            Token::EOC,
        ];
        assert_eq!(tokens, expected)
    }

    #[test]
    fn parse_strings() {
        let input = "echo \"hello world\"";
        let tokens = get_tokens(input).unwrap();
        let expected = vec![
            Token::Text("echo".into()),
            Token::Text("hello world".into()),
            Token::EOC,
        ];
        assert_eq!(tokens, expected);

        let input = "echo \'hello world\'";
        let tokens = get_tokens(input).unwrap();
        let expected = vec![
            Token::Text("echo".into()),
            Token::Text("hello world".into()),
            Token::EOC,
        ];
        assert_eq!(tokens, expected)
    }
}
