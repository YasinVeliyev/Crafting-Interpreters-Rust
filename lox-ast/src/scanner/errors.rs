#[derive(Debug)]
pub struct Error {
    pub line: usize,
    pub message: String,
}

pub enum Exception {
    HadError(bool),
}

impl Error {
    pub fn new(line: usize, message: &str, current: usize, source: &Vec<char>) -> Self {
        let message = format!(
            "{} {}\n{}\n{:>1$}\n",
            message,
            current,
            source.iter().collect::<String>(),
            '^'
        );

        Self { line, message: message }
    }
}

impl Exception {
    pub fn error(&mut self, line: usize, message: &str) {}
    pub fn report(err: Error, loc: &str) {
        eprintln!("[line {}] Error {} : {}", err.line, loc, err.message)
    }
}
