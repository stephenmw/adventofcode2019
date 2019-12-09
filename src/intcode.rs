use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn read_program<P: AsRef<Path>>(path: P) -> Result<Vec<i64>, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    raw.trim()
        .split_terminator(',')
        .try_fold(Vec::new(), |mut acc, x| {
            acc.push(x.parse()?);
            Ok(acc)
        })
}

pub fn execute_console_program(program: &[i64]) {
    fn get_input() -> i64 {
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

    let mut cpu = Computer::new(program);
    loop {
        match cpu.execute() {
            State::InputRequested => cpu.input(get_input()),
            State::Output(x) => println!("{}", x),
            State::Halted => break,
        };
    }
}

pub struct Computer {
    pub ram: [i64; 32768],
    pc: usize,
    relative_base: i64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    InputRequested,
    Output(i64),
    Halted,
}

impl Computer {
    pub fn new(program: &[i64]) -> Computer {
        let mut cpu = Computer {
            ram: [0; 32768],
            pc: 0,
            relative_base: 0,
        };

        cpu.ram[..program.len()].copy_from_slice(program);

        cpu
    }

    // Provides input. If state is not State::InputRequested, this function
    // panics.
    pub fn input(&mut self, i: i64) {
        if let Opcode::Input(m1) = parse_opcode(self.ram[self.pc]) {
            self.instruction_output(m1, 1, i);
            self.pc += 2;
        } else {
            panic!("input not requested");
        }
    }

    pub fn execute(&mut self) -> State {
        loop {
            let opcode = parse_opcode(self.ram[self.pc]);
            match opcode {
                Opcode::Add(m1, m2, o) => self.binary_op(|a, b| a + b, m1, m2, o),
                Opcode::Mul(m1, m2, o) => self.binary_op(|a, b| a * b, m1, m2, o),
                Opcode::Input(_) => return State::InputRequested,
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
                Opcode::LessThan(m1, m2, o) => {
                    self.binary_op(|a, b| if a < b { 1 } else { 0 }, m1, m2, o)
                }
                Opcode::Equals(m1, m2, o) => {
                    self.binary_op(|a, b| if a == b { 1 } else { 0 }, m1, m2, o)
                }
                Opcode::AdjustRelativeBase(m1) => {
                    self.relative_base += self.lookup_param(m1, 1);
                    self.pc += 2;
                }
                Opcode::Halt => return State::Halted,
            }
        }
    }

    fn binary_op<F>(&mut self, f: F, m1: ParameterMode, m2: ParameterMode, o: ParameterMode)
    where
        F: Fn(i64, i64) -> i64,
    {
        let a = self.lookup_param(m1, 1);
        let b = self.lookup_param(m2, 2);
        self.instruction_output(o, 3, f(a, b));
        self.pc += 4;
    }

    // Lookup an op parameter. Offset should start at 1.
    fn lookup_param(&self, mode: ParameterMode, offset: usize) -> i64 {
        let p = match mode {
            ParameterMode::Immediate => self.pc + offset,
            ParameterMode::Position => self.ram[self.pc + offset] as usize,
            ParameterMode::Relative => (self.relative_base + self.ram[self.pc + offset]) as usize,
        };

        self.ram[p]
    }

    // Writes output of an instruction.
    fn instruction_output(&mut self, mode: ParameterMode, offset: usize, value: i64) {
        let p = match mode {
            ParameterMode::Position => self.ram[self.pc + offset],
            ParameterMode::Relative => self.relative_base + self.ram[self.pc + offset],
            ParameterMode::Immediate => panic!("opcode output cannot be immediate value"),
        };

        self.ram[p as usize] = value;
    }
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn new(m: u8) -> ParameterMode {
        match m {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("bad ParameterMode"),
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    AdjustRelativeBase(ParameterMode),
    Halt,
}

fn parse_opcode(opcode: i64) -> Opcode {
    let instruction = opcode % 100;
    let m1 = ParameterMode::new((opcode / 100 % 10) as u8);
    let m2 = ParameterMode::new((opcode / 100 / 10 % 10) as u8);
    let m3 = ParameterMode::new((opcode / 100 / 10 / 10 % 10) as u8);

    match instruction {
        1 => Opcode::Add(m1, m2, m3),
        2 => Opcode::Mul(m1, m2, m3),
        3 => Opcode::Input(m1),
        4 => Opcode::Output(m1),
        5 => Opcode::JumpIfTrue(m1, m2),
        6 => Opcode::JumpIfFalse(m1, m2),
        7 => Opcode::LessThan(m1, m2, m3),
        8 => Opcode::Equals(m1, m2, m3),
        9 => Opcode::AdjustRelativeBase(m1),
        99 => Opcode::Halt,
        _ => panic!("bad Opcode"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exec_test(input: Vec<i64>, expected: Vec<i64>) {
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
