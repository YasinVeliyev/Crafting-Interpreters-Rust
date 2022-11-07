#[derive(Debug)]
pub struct Error {
    pub line: usize,
    pub message: String,
}

pub enum Exception {
    HadError(bool),
}

impl Error {
    pub fn new(line: usize, message: String) -> Self {
        Self { line, message: message }
    }
}

impl Exception {
    pub fn error(&mut self, line: usize, message: &str) {}
    pub fn report(err: Error, loc: &str) {
        eprintln!("[line {}] Error {} : {}", err.line, loc, err.message)
    }
}
