mod token;
mod scanner;

use std::{env, fs, io::{self, Write}};

use crate::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).unwrap();
    run(source);
}

fn run_prompt() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        run(input);
    }
}

fn run(source: String) {
    let scanner = Scanner::new(source);
    
    for token in scanner {
        println!("{:#?}", token);
    }
}
