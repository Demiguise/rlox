mod ast;
mod scanner;
mod tokens;

use anyhow::Result;
use ast::nodes::Literal;
use scanner::Scanner;
use std::{env, fs};
use text_io::read;

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
        Ok(_) => {}
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
            Ok(_) => {}
            Err(e) => {
                println!("Error: {:?}", e)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let lit = ast::nodes::Expr::Literal(ast::nodes::Literal::String("123".to_owned()));
    let unary = ast::nodes::Expr::Unary(
        tokens::Token::create(tokens::TokenType::Minus, "-".to_owned(), 1),
        Box::new(lit),
    );

    let star = tokens::Token::create(tokens::TokenType::Star, "*".to_owned(), 1);

    let group_lit = ast::nodes::Expr::Literal(ast::nodes::Literal::String("45.67".to_owned()));
    let grouping = ast::nodes::Expr::Grouping(Box::new(group_lit));

    let expr = ast::nodes::Expr::Binary(Box::new(unary), star, Box::new(grouping));

    let printer = ast::ASTPrinter::create();
    println!("{}", printer.print(&expr));

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => println!("Wrong usage, try again"),
    }
}
