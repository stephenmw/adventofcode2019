use std::io;
use std::io::Write;

use crate::intcode;
use crate::intcode::State;

pub fn main() {
    let program = intcode::read_program("data/day5.txt").expect("failed to read program");
    execute_terminal_program(program);
}

fn execute_terminal_program(program: Vec<i32>) {
    let mut cpu = intcode::Computer::new(program);
    loop {
        match cpu.execute() {
            State::InputRequested => cpu.input(get_input()),
            State::Output(x) => println!("{}", x),
            State::Halted => break,
            State::Start => unreachable!("State::Start never returned"),
        };
    }
}

fn get_input() -> i32 {
    let mut buf = String::new();
    loop {
        buf.clear();
        print!("> ");
        let _ = io::stdout().lock().flush();
        let result = io::stdin().read_line(&mut buf);
        if result.is_err() {
            println!();
            continue;
        }

        let trimmed = buf.trim();
        match trimmed.parse() {
            Ok(x) => return x,
            Err(_) => println!("bad input"),
        };
    }
}
