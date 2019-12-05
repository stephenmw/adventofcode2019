use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn main() {
    let rom = read_integers("data/day5.txt").expect("failed to read program");
    let cpu = IntcodeComputer::new(rom);
    cpu.execute();
}

struct IntcodeComputer {
    rom: Vec<i32>,
}

impl IntcodeComputer {
    fn new(rom: Vec<i32>) -> IntcodeComputer {
        IntcodeComputer { rom: rom }
    }

    fn execute(&self) -> Vec<i32> {
        let mut ram = self.rom.clone();
        let mut pc = 0;

        loop {
            let opcode = parse_opcode(ram[pc]);
            match opcode {
                Opcode::Add(m1, m2) => {
                    let a = lookup_param(&ram, m1, pc + 1);
                    let b = lookup_param(&ram, m2, pc + 2);
                    let t = ram[pc + 3] as usize;
                    ram[t] = a + b;
                    pc += 4;
                }
                Opcode::Mul(m1, m2) => {
                    let a = lookup_param(&ram, m1, pc + 1);
                    let b = lookup_param(&ram, m2, pc + 2);
                    let t = ram[pc + 3] as usize;
                    ram[t] = a * b;
                    pc += 4;
                }
                Opcode::Input => {
                    let t = ram[pc + 1] as usize;
                    ram[t] = get_input();
                    pc += 2;
                }
                Opcode::Output(m1) => {
                    println!("{}", lookup_param(&ram, m1, pc + 1));
                    pc += 2;
                }
                Opcode::Halt => break,
            }
        }

        return ram
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

fn lookup_param(ram: &[i32], mode: ParameterMode, index: usize) -> i32 {
    match mode {
        ParameterMode::Immediate => ram[index],
        ParameterMode::Position => ram[ram[index] as usize],
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
    Halt,
}

fn parse_opcode(opcode: i32) -> Opcode {
    let mut opcode = opcode;
    let instruction = opcode % 100;
    opcode /= 100;
    let m1 = ParameterMode::new((opcode % 10) as u8);
    opcode /= 10;
    let m2 = ParameterMode::new((opcode % 10) as u8);

    match instruction {
        1 => Opcode::Add(m1, m2),
        2 => Opcode::Mul(m1, m2),
        3 => Opcode::Input,
        4 => Opcode::Output(m1),
        99 => Opcode::Halt,
        _ => panic!("bad Opcode"),
    }
}

fn read_integers<P: AsRef<Path>>(path: P) -> Result<Vec<i32>, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    raw.split_terminator(',')
        .try_fold(Vec::new(), |mut acc, x| {
            acc.push(x.parse()?);
            Ok(acc)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exec_test(input: Vec<i32>, expected: Vec<i32>) {
        assert_eq!(IntcodeComputer::new(input).execute(), expected)
    }

    #[test]
    fn test_intcode() {
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
