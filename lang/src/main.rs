use std::env;
use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};

use lexical_analysis::Lexer;
use crate::ast::generate_ast::generate_ast;

pub mod lexical_analysis;
pub mod ast;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 => run_prompt(),
        2 => run_file(&args[1]),
        3 => generate_ast(&args[2]),
        _ => println!("Usage: 0x6b73746b [script]\nAST generation: 0x6b73746b ast [output directory]"),
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
