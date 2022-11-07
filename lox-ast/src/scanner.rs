use std::env;

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

pub mod errors;
pub mod tokens;

use errors::*;
use tokens::{object::*, types::*};

use self::tokens::Token;

pub fn run_file(path: &String) -> io::Result<()> {
    let file = File::open(path)?;
    let mut buffer = String::new();
    let result = BufReader::new(file).read_to_string(&mut buffer);
    run(buffer).unwrap();
    Ok(())
}
pub fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    io::stdout().flush();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                if !line.is_empty() {
                    run(line).unwrap();
                }
            }
            Err(err) => panic!("Error"),
        }
    }
}

pub fn run(source: String) -> Result<(), Error> {
    let mut scanner = Scanner::new(source)?;
    match scanner.scan_tokens() {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err.message);
            std::process::exit(65)
        }
    };
    println!("{:?}", scanner.get_tokens());
    Ok(())
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Result<Self, Error> {
        Ok(Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        })
    }
    fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    fn scan_tokens(&mut self) -> Result<(), Error> {
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(_) => {}
                Err(err) => {
                    return Err(err);
                }
            };
        }
        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line));
        Ok(())
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), Error> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftRoundBracket, None),
            ')' => self.add_token(TokenType::RightRoundBracket, None),
            '{' => self.add_token(TokenType::LeftCurlyBracket, None),
            '}' => self.add_token(TokenType::RightCurlyBracket, None),
            '[' => self.add_token(TokenType::LeftSquareBracket, None),
            ']' => self.add_token(TokenType::RightSquareBracket, None),
            '+' => self.add_token(TokenType::Plus, None),
            '-' => self.add_token(TokenType::Minus, None),
            '/' => self.add_token(TokenType::Slash, None),
            '*' => self.add_token(TokenType::Star, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            ';' => self.add_token(TokenType::SemiColon, None),
            '!' => {
                let token = if self.next_is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token, None)
            }
            '=' => {
                let token = if self.next_is_match('=') {
                    TokenType::Equals
                } else {
                    TokenType::Assign
                };
                self.add_token(token, None)
            }
            '>' => {
                let token = if self.next_is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token, None)
            }
            '<' => {
                let token = if self.next_is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token, None)
            }
            '\\' => Ok(if self.next_is_match('n') {
                self.line += 1;
            } else if self.next_is_match('t') || self.next_is_match('r') {
                self.advance();
            }),
            '/' => self.add_token(TokenType::Slash, None),

            _ => Err(Error::new(
                self.line,
                format!(
                    "Unexpected character at {}\n{}\n{:>0$}\n",
                    self.current, self.source, '^'
                ),
            )),
        }
    }

    fn advance(&mut self) -> char {
        let current = self.current;
        self.current += 1;
        self.source.chars().nth(current).unwrap()
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Object>) -> Result<(), Error> {
        let lexeme = &self.source[self.start..self.current].to_owned();
        self.tokens.push(Token::new(ttype, lexeme, literal, self.line));
        Ok(())
    }

    fn next_is_match(&mut self, expected: char) -> bool {
        match self.source.chars().nth(self.current) {
            Some(ch) if ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }
}
