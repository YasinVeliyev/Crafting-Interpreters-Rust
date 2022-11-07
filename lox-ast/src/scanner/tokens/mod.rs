use std::fmt;

pub mod object;
use self::object::*;

use self::types::TokenType;

pub mod types;

pub struct Token {
    pub ttype: types::TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(ttype: types::TokenType, lexeme: &str, literal: Option<Object>, line: usize) -> Self {
        Self {
            ttype,
            lexeme: lexeme.to_owned(),
            literal,
            line,
        }
    }
}
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.ttype,
            self.lexeme,
            if let Some(ref literal) = self.literal {
                literal
            } else {
                &Object::Nil
            },
        )
    }
}
