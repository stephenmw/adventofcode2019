use crate::intcode;

pub fn main() {
    let program = intcode::read_program("data/day05.txt").expect("failed to read program");
    intcode::execute_console_program(&program);
}
