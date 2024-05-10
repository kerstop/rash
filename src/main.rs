use std::{
    io::{BufRead, Write},
    iter::Peekable,
    str::Chars,
};

use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::streaming::escaped_transform,
    character::complete::{char, multispace1, newline, none_of},
    combinator::{opt, value},
    multi::{fold_many1, many0},
    sequence::preceded,
    IResult, InputTakeAtPosition,
};

fn main() -> anyhow::Result<()> {
    let mut input = std::io::stdin().lock();
    let mut output = std::io::stdout().lock();

    loop {
        print!(">>> ");
        output.flush()?;

        let mut command = String::new();
        input.read_line(&mut command)?;

        let (_, args) = parse_cmd(&command).unwrap();

        if args.len() == 0 {
            continue;
        }
        match *args.get(0).unwrap() {
            "echo" => {
                let iter = args.iter().skip(1);
                for arg in iter {
                    write!(&mut output, "{arg} ")?;
                }
                write!(&mut output, "\n")?;
                output.flush()?;
            }
            "quit" => {
                break;
            }
            _ => todo!(),
        }
    }
    Ok(())
}

/// parse until an unescaped newline or a semicolon is encountered
fn parse_cmd(input: &str) -> IResult<&str, Vec<&str>> {
    split_on_whitespace(input)
}

fn split_on_whitespace(input: &str) -> IResult<&str, Vec<&str>> {
    many0(preceded(opt(multispace1), not_space1))(input)
}

fn not_space1(input: &str) -> IResult<&str, &str> {
    input.split_at_position1_complete(|c| c.is_whitespace(), nom::error::ErrorKind::Alpha)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_arg_parsing() {
        assert_eq!(
            parse_cmd("echo hello world").unwrap().1,
            vec!["echo", "hello", "world"]
        );
    }
}
