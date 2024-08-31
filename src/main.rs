mod parser;
mod scanner;
mod token;
mod exception;

use clap::Parser;
use token::Token;
use std::{ env, fs, path::PathBuf, process, result };

// use parser::Parser;
use scanner::Scanner;


#[derive(Parser, Debug)]
struct Args {
    file: PathBuf,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // let args = Args::parse();

    if args.len() > 1 {
        dbg!("Usage: totx [script]");
        process::exit(64);
    }

    let source = fs::read_to_string(&args[1]).expect("Unable to read the file");

    let scanner = Scanner::new(&source);
    let tokens = scanner;

    // else {
    //     run_prompt();
    // }
}
