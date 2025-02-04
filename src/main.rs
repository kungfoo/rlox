use std::fs;
use std::io::{self, BufRead};
use std::thread::scope;
use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script]");
        exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(filename: &str) {
    let contents =
        fs::read_to_string(filename).expect("Should have been able to read the script file");
    run(&contents);
}

fn run_prompt() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Unable to read line from stdin");
        run(&line);
    }
}

fn run(script: &str) {
    if script.len() > 0 {
        println!("Running {}", script);
    }
}
