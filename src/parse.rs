use crate::sheet::{Cell, Formula, Range};
use anyhow::{bail, Result};
use logos::{Lexer, Logos};

#[derive(Debug, PartialEq)]
pub enum ParseError {
    Error,
}

fn string_token(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    Some(slice[1..slice.len() - 1].into())
}
#[derive(Logos, Debug, Clone, PartialEq, Hash)]
pub enum Token {
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(",")]
    Comma,
    #[token("=")]
    Equals,
    #[token(":")]
    Colon,
    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Number(i64),
    #[regex("[a-zA-Z][a-zA-Z_]*", |lex| lex.slice().parse())]
    Identifier(String),

    #[token(r#""[^"]*""#, string_token)]
    String(String),
}

pub fn letter_to_index(letter: &str) -> u32 {
    let mut column = 0;
    let length = letter.len();
    for (i, c) in letter.chars().enumerate() {
        column += (c as u32 - 64) * 26u32.pow(length as u32 - i as u32 - 1)
    }
    return column - 1;
}

pub fn index_to_letter(mut column: u32) -> String {
    column += 1;
    let mut letter: String = String::new();

    while column > 0 {
        let temp: u32 = (column - 1) % 26;
        letter = format!("{}{}", std::char::from_u32(temp + 65).unwrap(), letter);
        column = (column - temp - 1) / 26;
    }
    return letter;
}

fn expect_token(lex: &mut Vec<Token>, token: Token) -> Result<()> {
    match lex.pop() {
        Some(t) if t == token => Ok(()),
        _ => bail!("Expected token: {:?}", token),
    }
}

fn parse_expr(lex: &mut Vec<Token>) -> Result<Formula> {
    println!("Parsing expr, {:?}", lex);
    match lex.pop() {
        Some(Token::Identifier(name)) => {
            match lex.pop() {
                Some(Token::LParen) => {
                    let mut exprs = vec![];

                    // check for no arguments
                    match lex.last() {
                        Some(Token::RParen) => {
                            lex.pop();
                            return Ok(Formula::Function(name, vec![]));
                        }
                        _ => (),
                    }

                    // otherwise parse arguments
                    loop {
                        exprs.push(parse_expr(lex)?);
                        match lex.pop() {
                            Some(Token::Comma) => continue,
                            Some(Token::RParen) => {
                                return Ok(Formula::Function(name, exprs));
                            }
                            _ => bail!("Expected comma or RParen"),
                        };
                    }
                }
                Some(Token::Number(n)) => {
                    expect_token(lex, Token::Colon)?;
                    let column1 = name;
                    let row1 = n - 1;
                    let column2 = match lex.pop() {
                        Some(Token::Identifier(id)) => id,
                        _ => bail!("Expected identifier"),
                    };
                    let row2 = match lex.pop() {
                        Some(Token::Number(n)) => n - 1,
                        _ => bail!("Expected number"),
                    };

                    let column1 = letter_to_index(&column1);
                    let column2 = letter_to_index(&column2);

                    let range = Range::new(
                        (column1 as usize, row1 as usize),
                        (column2 as usize, row2 as usize),
                    );

                    Ok(Formula::Range(range))
                }
                _ => bail!("Expected LParen or Number"),
            }
        }
        Some(Token::String(str)) => Ok(Formula::Text(str)),
        t => bail!("err: {:?}", t),
    }
}

fn parse_cell(lex: &mut Vec<Token>) -> Result<Cell> {
    match lex.pop() {
        Some(Token::Equals) => Ok(Cell::Formula(parse_expr(lex)?)),
        Some(Token::String(str)) => Ok(Cell::Text(str)),
        e => bail!("Expected equals or string, but got {:?}", e),
    }
}

pub fn parse(s: &str) -> Result<Cell> {
    println!("Parsing: {:?}", s);
    let mut lex = Token::lexer(s)
        .into_iter()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();

    return parse_cell(&mut lex);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "=SUM(A1:A2)";
        let expected_output = Cell::Formula(Formula::Function(
            "SUM".to_string(),
            vec![Formula::Range(Range {
                start: (0, 0),
                end: (0, 1),
            })],
        ));
        assert_eq!(super::parse(s).unwrap(), expected_output);
    }
}
