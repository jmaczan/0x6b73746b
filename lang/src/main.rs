use std::env;
use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};

use lexical_analysis::Lexer;

pub mod lexical_analysis;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => run_file(&args[1]),
        1 => run_prompt(),
        0 => run_prompt(),
        _ => println!("Usage: 0x6b73746b [script]"),
    }
}

fn run_file(path: &String) {
    run(read_to_string(path).expect("Failed to read from file."));
}

fn run(source: String) {
    let mut lexer = Lexer::new(source);
    lexer.scan_tokens();
    println!("{:?}", lexer.tokens);
}

fn run_prompt() {
    let input = stdin();
    let mut reader = stdout();

    loop {
        print!(">");
        reader.flush().unwrap();

        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(_) => run(line),
            Err(_) => break,
        }
    }
}

pub fn error(line: u8, message: String) {
    report(line, "".to_owned(), message);
}

fn report(line: u8, source: String, message: String) {
    println!("[line: {}] Error {}: {}", line, source, message);
}
