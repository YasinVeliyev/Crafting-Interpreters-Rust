use std::fmt;

#[derive(Debug)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Object::*;
        match self {
            Num(number) => write!(f, "{}", number),
            Str(string) => write!(f, "{}", string),
            Nil => write!(f, "{}", "None"),
            True => write!(f, "{}", "True"),
            False => write!(f, "{}", "False"),
        }
    }
}
