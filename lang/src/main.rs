use std::env;
use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};

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
    println!("{}", source);
}

fn run_prompt() {
    let input = stdin();
    let mut reader = stdout();

    loop {
        print!(">");
        reader.flush().unwrap();

        let mut line = String::new();
        input.read_line(&mut line).unwrap();

        run(line);
    }
}
