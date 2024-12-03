mod ast;
mod scanner;
mod token;
mod exception;
mod parser;

use clap::Parser;
use std::{ env, fs, path::PathBuf, process };

use scanner::Scanner;


#[derive(Parser, Debug)]
struct Args {
    file: PathBuf,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let args = Args::parse();

    if args.len() > 2 {
        dbg!("Usage: totx [script]");
        process::exit(64);
    }

    let source = fs::read_to_string(&"input.isi").expect("Unable to read the file");

    let mut scanner = Scanner::new(&source);
    let token = scanner.scan_tokens().unwrap();
    let mut parser = parser::Parser::new(token);
    let parse = parser.parser();
    dbg!(parse);
}
