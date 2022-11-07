#[derive(Debug)]
pub struct Error<'a> {
    pub line: usize,
    pub message: &'a str,
}

pub enum Exception {
    HadError(bool),
}

impl Exception {
    pub fn error(&mut self, line: usize, message: &str) {}
    pub fn report(err: Error, loc: &str) {
        eprintln!("[line {}] Error {} : {}", err.line, loc, err.message)
    }
}
