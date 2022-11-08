mod scanner;
use scanner::*;
use std::cmp::Ordering;
use std::env::args;
fn main() {
    let args = args().skip(1).collect::<Vec<String>>();
    match args.len().cmp(&1) {
        Ordering::Greater => {
            println!("Usage: jlox [script]");
            std::process::exit(64);
        }
        Ordering::Equal => run_file(&args[0]).unwrap(),
        Ordering::Less => run_prompt(),
    }
}
