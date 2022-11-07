mod scanner;
use scanner::*;
use std::env::args;

fn main() {
    let args = args().skip(1).collect::<Vec<String>>();
    if args.len() > 1 {
        println!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        run_file(&args[0]).unwrap();
    } else {
        run_prompt();
    }
}
