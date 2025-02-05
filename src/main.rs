use std::fs;
use std::io::{self, BufRead};
use std::{env, process::exit};

mod scanner;

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
    if lox::state().has_error() {
        exit(65);
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Unable to read line from stdin");
        run(&line);
        lox::state().reset();
    }
}

fn run(script: &str) {
    if script.len() > 0 {
        let scanner = &mut scanner::Scanner::new(script);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token);
        }
    }
}

mod lox {
    use std::sync::{Mutex, MutexGuard};

    #[derive(Debug)]
    pub struct State {
        has_error: bool,
    }

    impl State {
        pub fn has_error(&self) -> bool {
            self.has_error
        }

        pub fn reset(&mut self) {
            self.has_error = false;
        }
    }

    static STATE: Mutex<State> = Mutex::new(State { has_error: false });

    pub fn state() -> MutexGuard<'static, State> {
        STATE.lock().unwrap()
    }

    pub fn error(line: usize, message: &str) -> () {
        report(line, "", message);
    }

    pub fn report(line: usize, _where: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, _where, message);
        let mut state = state();
        state.has_error = true;
    }
}
