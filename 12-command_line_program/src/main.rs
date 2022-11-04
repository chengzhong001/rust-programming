#![allow(unused)]
use command_line_program;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = command_line_program::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = command_line_program::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
