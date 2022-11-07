use std::env;
use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

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
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    run(line).unwrap();
                }
            }
            Err(err) => panic!("Error"),
        }
    }
}

pub fn run(source: String) -> Result<(), Error> {
    let scanner = Scanner::new(source)?;
    let tokens = scanner.get_tokens();
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

    fn scan_tokens(&mut self) {
        while self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenType::Eof, "", None, self.line))
    }
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
    }

    fn advance(&mut self) -> char {
        let current = self.current;
        self.current += 1;
        self.source.chars().nth(current).unwrap()
    }

    fn add_token(&mut self, ttype: TokenType, lexeme: &str, literal: Option<Object>) {
        self.tokens.push(Token::new(ttype, lexeme, literal, self.line))
    }
}
