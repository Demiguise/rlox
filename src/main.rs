mod scanner;
mod tokens;

use std::{env, fs};
use scanner::Scanner;
use text_io::read;

fn run(input: String) {
    let scanner = Scanner::create(input);
}

fn run_file(file_path: &String) {
    println!("Attempting to load file [{}]", file_path);
    let contents = fs::read_to_string(file_path).expect("Unable to read file");
    run(contents)
}

fn run_prompt() {
    println!("Running Interpreter");
    loop {
        print!("> ");
        let line: String = read!("{}\n");
        run(line)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => println!("Wrong usage, try again"),
    }
}
