mod ast;
mod scanner;
mod token;
mod exception;
mod parser;

use clap::Parser;
use std::{ env, fs, path::PathBuf, process };

// use parser::Parser;
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

    let scanner = Scanner::new(&source).scan_tokens();

    // println!("{:?}", scanner);
    // let tokens = scanner;

    // else {
    //     run_prompt();
    // }
}
