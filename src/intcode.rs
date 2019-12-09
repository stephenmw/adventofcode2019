use std::error::Error;
use std::fs;
use std::path::Path;

pub fn read_program<P: AsRef<Path>>(path: P) -> Result<Vec<i32>, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    raw.split_terminator(',')
        .try_fold(Vec::new(), |mut acc, x| {
            acc.push(x.parse()?);
            Ok(acc)
        })
}

pub struct Computer {
    pub ram: Vec<i32>,
    pc: usize,
    state: State,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Start,
    InputRequested,
    Output(i32),
    Halted,
}

impl Computer {
    pub fn new(ram: Vec<i32>) -> Computer {
        Computer {
            ram: ram,
            pc: 0,
            state: State::Start,
        }
    }

    // Provides input. If state is not State::InputRequested, this function
    // panics.
    pub fn input(&mut self, i: i32) {
        if self.state != State::InputRequested {
            panic!("input not requested");
        }

        let t = self.ram[self.pc + 1] as usize;
        self.ram[t] = i;
        self.pc += 2;

        self.state = State::Start;
    }

    pub fn execute(&mut self) -> State {
        self.state = self.execute_();
        self.state
    }

    fn execute_(&mut self) -> State {
        loop {
            let opcode = parse_opcode(self.ram[self.pc]);
            match opcode {
                Opcode::Add(m1, m2) => self.binary_op(|a, b| a + b, m1, m2),
                Opcode::Mul(m1, m2) => self.binary_op(|a, b| a * b, m1, m2),
                Opcode::Input => return State::InputRequested,
                Opcode::Output(m1) => {
                    let out = self.lookup_param(m1, 1);
                    self.pc += 2;
                    return State::Output(out);
                }
                Opcode::JumpIfTrue(m1, m2) => {
                    let cond = self.lookup_param(m1, 1) != 0;
                    let target = self.lookup_param(m2, 2);
                    if cond {
                        self.pc = target as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Opcode::JumpIfFalse(m1, m2) => {
                    let cond = self.lookup_param(m1, 1) != 0;
                    let target = self.lookup_param(m2, 2);
                    if !cond {
                        self.pc = target as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Opcode::LessThan(m1, m2) => {
                    self.binary_op(|a, b| if a < b { 1 } else { 0 }, m1, m2)
                }
                Opcode::Equals(m1, m2) => self.binary_op(|a, b| if a == b { 1 } else { 0 }, m1, m2),
                Opcode::Halt => return State::Halted,
            }
        }
    }

    fn binary_op<F>(&mut self, f: F, m1: ParameterMode, m2: ParameterMode)
    where
        F: Fn(i32, i32) -> i32,
    {
        let a = self.lookup_param(m1, 1);
        let b = self.lookup_param(m2, 2);
        let t = self.ram[self.pc + 3] as usize;
        self.ram[t] = f(a, b);
        self.pc += 4;
    }

    // Lookup an op parameter. Offset should start at 1.
    fn lookup_param(&self, mode: ParameterMode, offset: usize) -> i32 {
        match mode {
            ParameterMode::Immediate => self.ram[self.pc + offset],
            ParameterMode::Position => self.ram[self.ram[self.pc + offset] as usize],
        }
    }
}

enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn new(m: u8) -> ParameterMode {
        match m {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("bad ParameterMode"),
        }
    }
}

enum Opcode {
    Add(ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode),
    Input,
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),
    Halt,
}

fn parse_opcode(opcode: i32) -> Opcode {
    let instruction = opcode % 100;
    let m1 = ParameterMode::new((opcode / 100 % 10) as u8);
    let m2 = ParameterMode::new((opcode / 100 / 10 % 10) as u8);

    match instruction {
        1 => Opcode::Add(m1, m2),
        2 => Opcode::Mul(m1, m2),
        3 => Opcode::Input,
        4 => Opcode::Output(m1),
        5 => Opcode::JumpIfTrue(m1, m2),
        6 => Opcode::JumpIfFalse(m1, m2),
        7 => Opcode::LessThan(m1, m2),
        8 => Opcode::Equals(m1, m2),
        99 => Opcode::Halt,
        _ => panic!("bad Opcode"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exec_test(input: Vec<i32>, expected: Vec<i32>) {
        let mem = input.clone();
        let mut cpu = Computer::new(mem);
        cpu.execute();
        assert_eq!(cpu.ram, expected);
    }

    #[test]
    fn test_execute() {
        exec_test(vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
        exec_test(vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
        exec_test(vec![2, 4, 4, 5, 99, 0], vec![2, 4, 4, 5, 99, 9801]);
        exec_test(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
        exec_test(
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
    }
}
