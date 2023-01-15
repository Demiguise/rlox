mod scanner;
mod tokens;

use scanner::Scanner;
use std::{env, fs};
use text_io::read;
use anyhow::Result;

fn run(input: String) -> Result<()> {
    let mut scanner = Scanner::create(input);
    let tokens = scanner.scan_tokens()?;

    println!("{:?}", tokens);
    Ok(())
}

fn run_file(file_path: &String) {
    println!("Attempting to load file [{}]", file_path);
    let contents = fs::read_to_string(file_path).expect("Unable to read file");
    match run(contents) {
        Ok(_) => {},
        Err(e) => {
            println!("{:?}", e)
        }
    }
}

fn run_prompt() {
    println!("Running Interpreter");
    loop {
        print!("> ");
        let line: String = read!("{}\n");
        match run(line) {
            Ok(_) => {},
            Err(e) => {
                println!("Error: {:?}", e)
            }
        }
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
