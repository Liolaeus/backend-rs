use core::fmt;
use std::num::ParseFloatError;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LPar,
    RPar,
    Eof,
}

#[derive(Debug)]
pub enum ExpError {
    InvalidToken(String),
    ExpectedToken(String),
    ParseNum(ParseFloatError),
    DivZero,
}

impl fmt::Display for ExpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExpError::ExpectedToken(msg) | ExpError::InvalidToken(msg) => write!(f, "{}", msg),
            ExpError::ParseNum(msg) => write!(f, "{}", msg),
            ExpError::DivZero => write!(f, "division by zero"),
        }
    }
}

impl IntoResponse for ExpError {
    fn into_response(self) -> Response {
        let body = self.to_string();
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}

struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(s: &str) -> Self {
        Self {
            input: s.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.pos += 1;
        }
    }

    fn number(&mut self, first: char) -> Result<Token, ExpError> {
        let mut s = String::new();
        s.push(first);
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                s.push(c);
                self.pos += 1;
            } else {
                break;
            }
        }
        let num = s.parse::<f64>().map_err(ExpError::ParseNum)?;
        Ok(Token::Number(num))
    }

    fn next(&mut self) -> Result<Token, ExpError> {
        self.skip_whitespace();

        let ch = self.peek();
        if ch.is_some() {
            self.pos += 1;
        }
        match ch {
            Some(c) if c.is_ascii_digit() || c == '.' => self.number(c),
            Some('+') => Ok(Token::Plus),
            Some('-') => Ok(Token::Minus),
            Some('*') => Ok(Token::Star),
            Some('/') => Ok(Token::Slash),
            Some('(') => Ok(Token::LPar),
            Some(')') => Ok(Token::RPar),
            None => Ok(Token::Eof),
            Some(_) => Err(ExpError::InvalidToken(format!(
                "Unexpected token: {:?}",
                ch
            ))),
        }
    }
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }
    fn step(&mut self) -> Token {
        let t = self.peek().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        t
    }

    fn parse(&mut self) -> Result<f64, ExpError> {
        let val = self.parse_expr()?;
        match self.peek() {
            Token::Eof => Ok(val),
            t => Err(ExpError::InvalidToken(format!("Unexpected token: {:?}", t))),
        }
    }

    // handle addition, soustraction
    fn parse_expr(&mut self) -> Result<f64, ExpError> {
        let mut lhs = self.parse_term()?;
        loop {
            match self.peek() {
                Token::Plus => {
                    self.step();
                    let rhs = self.parse_term()?;
                    lhs += rhs;
                }
                Token::Minus => {
                    self.step();
                    let rhs = self.parse_term()?;
                    lhs -= rhs;
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    // handle mult, div
    fn parse_term(&mut self) -> Result<f64, ExpError> {
        let mut lhs = self.parse_factor()?;
        loop {
            match self.peek() {
                Token::Star => {
                    self.step();
                    let rhs = self.parse_factor()?;
                    lhs *= rhs;
                }
                Token::Slash => {
                    self.step();
                    let rhs = self.parse_factor()?;
                    if rhs == 0.0 {
                        return Err(ExpError::DivZero);
                    }
                    lhs /= rhs;
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    // handle numbers, parenthesis, unary negation
    fn parse_factor(&mut self) -> Result<f64, ExpError> {
        match self.peek().clone() {
            Token::Number(n) => {
                self.step();
                Ok(n)
            }
            Token::LPar => {
                self.step();
                let v = self.parse_expr()?;
                match self.peek() {
                    Token::RPar => {
                        self.step();
                        Ok(v)
                    }
                    _ => Err(ExpError::ExpectedToken(
                        "Expected closing parenthesis".to_string(),
                    )),
                }
            }
            Token::Minus => {
                self.step();
                let v = self.parse_factor()?;
                Ok(-v)
            }
            t => Err(ExpError::InvalidToken(format!("Unexpected token: {:?}", t))),
        }
    }
}

pub fn evaluate_expression(s: &str) -> Result<f64, ExpError> {
    let mut lx = Lexer::new(s);
    let mut tokens = Vec::new();
    loop {
        let t = lx.next()?;
        if t == Token::Eof {
            tokens.push(t);
            break;
        }
        tokens.push(t);
    }
    let mut p = Parser::new(tokens);
    p.parse()
}
