use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};

pub mod errors;
mod keywords;
pub mod tokens;

use errors::*;
use tokens::{object::*, types::*};

use self::keywords::*;
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
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Result<Self, Error> {
        Ok(Self {
            source: source.chars().collect(),
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
        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));
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
            '/' => {
                if self.next_is_match('/') {
                    // A comment goes until the end of the line
                    while let Some(ch) = self.peek() {
                        if ch != '\n' {
                            // self.advance();
                            self.current += 1;
                        } else {
                            break;
                        }
                    }
                } else if self.next_is_match('*') {
                    // block comment start
                    println!("Hiiiiiiiiiiiiiiii");
                    return self.scan_comment();
                } else {
                    self.add_token(TokenType::Slash, None).unwrap();
                };
                Ok(())
            }
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
            '\\' => {
                if self.next_is_match('n') {
                    self.line += 1;
                } else if self.next_is_match('t') || self.next_is_match('r') {
                    self.current += 1;
                };
                Ok(())
            }

            ' ' | '\r' | '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            }
            '"' => self.string(),
            '0'..='9' => self.number(),
            _ => {
                if c.is_ascii_alphabetic() {
                    self.identifier()
                } else {
                    Err(Error::new(
                        self.line,
                        "Unexpected character at",
                        self.current,
                        &self.source,
                    ))
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        let current = self.current;
        self.current += 1;
        self.source.get(current).copied().unwrap()
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Object>) -> Result<(), Error> {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token::new(ttype, lexeme, literal, self.line));
        Ok(())
    }

    fn next_is_match(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if ch == &expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn string(&mut self) -> Result<(), Error> {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => break,
                '\n' => self.line += 1,
                _ => {}
            }
            // self.advance()
            self.current += 1;
        }

        if self.is_at_end() {
            return Err(Error::new(self.line, "Unterminated string", self.current, &self.source));
        }

        // self.advance();
        self.current += 1;
        let value: String = self.source[self.start..self.current].iter().collect();
        self.add_token(TokenType::String, Some(Object::Str(value)))
    }

    fn peek(&mut self) -> Option<char> {
        self.source.get(self.current).copied()
    }
    fn number(&mut self) -> Result<(), Error> {
        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() && !self.peek_next().is_ascii_alphabetic() {
                //self.advance()
                self.current += 1;
            } else if ch == '.' && self.peek_next().is_ascii_digit() {
                self.current += 1
            } else if self.peek_next() != '.' && (self.peek_next() == '"' || self.peek_next().is_ascii_alphabetic()) {
                return Err(Error::new(
                    self.line,
                    "SyntaxError: invalid syntax",
                    self.current + 2,
                    &self.source,
                ));
            } else {
                break;
            }
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        let num: f64 = value.parse().unwrap();
        self.add_token(TokenType::Number, Some(Object::Num(num)))
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\n'
        } else {
            *self.source.get(self.current + 1).unwrap()
        }
    }
    fn identifier(&mut self) -> Result<(), Error> {
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() {
                self.current += 1;
            } else {
                break;
            }
            // self.advance();
        }
        if let Some(ttype) = Scanner::keywords("keyword") {
            self.add_token(ttype, None)
        } else {
            self.add_token(TokenType::Identifier, None)
        }
    }
    fn scan_comment(&mut self) -> Result<(), Error> {
        while !self.next_is_match('*') && !(self.peek_next() == '/') {
            if self.current >= self.source.len() {
                return Err(Error::new(
                    self.line,
                    "Uncaught SyntaxError: Invalid or unexpected token",
                    self.current,
                    &self.source,
                ));
            }
            self.current += 1;
        }
        self.current += 1;
        Ok(())
    }
    fn keywords(keyword: &str) -> Option<TokenType> {
        match keyword {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            "break" => Some(TokenType::Break),
            _ => None,
        }
    }
}
