use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::cmp::max;

pub fn main() {
    let rom = read_integers("data/day7.txt").expect("failed to read program");
    println!("Part 1: {}", max_5amp_signal(rom.clone()));
}

fn max_5amp_signal(rom: Vec<i32>) -> i32 {
    let mut ret = 0;

    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        let candidate = &vec![a,b,c,d,e];
                        if only_one_used(candidate.clone()) {
                            ret = max(ret, amp_series(rom.clone(), &vec![a,b,c,d,e]));
                        }
                    }
                }
            }
        }
    }

    ret
}

fn only_one_used(xs: Vec<i32>) -> bool {
    let mut xs = xs;
    xs.sort();
    xs[..].windows(2).all(|x| x[0] != x[1])
}

fn amp_series(rom: Vec<i32>, phases: &[i32]) -> i32 {
    let mut output = 0;
    for phase in phases {
        output = run_amplifier(rom.clone(), *phase, output)
    }
    output
}

fn run_amplifier(rom: Vec<i32>, phase: i32, input: i32) -> i32 {
    let mut ret = 0;
    let i = mock_input(vec![input, phase]);
    let o = |x| ret = x;
    let mut cpu = IntcodeComputer::with_io(rom, i, o);
    cpu.execute();

    ret
}

fn mock_input(stack: Vec<i32>) -> impl FnMut() -> i32 {
    let mut input = stack;
    Box::new( move || input.pop().unwrap())
}

struct IntcodeComputer<I: FnMut() -> i32, O: FnMut(i32)> {
    rom: Vec<i32>,
    input: I,
    output: O,
}

impl<I: FnMut() -> i32, O: FnMut(i32)> IntcodeComputer<I, O> {
    fn new(rom: Vec<i32>) -> IntcodeComputer<impl FnMut() -> i32, impl FnMut(i32)> {
        IntcodeComputer::with_io(rom, get_input, |x| println!("{}", x))
    }

    fn with_io(rom: Vec<i32>, input: I, output: O) -> IntcodeComputer<I, O> {
        IntcodeComputer {
            rom: rom,
            input: input,
            output: output,
        }
    }

    fn execute(&mut self) -> Vec<i32> {
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
                    ram[t] = (self.input)();
                    pc += 2;
                }
                Opcode::Output(m1) => {
                    (self.output)(lookup_param(&ram, m1, pc + 1));
                    pc += 2;
                }
                Opcode::JumpIfTrue(m1, m2) => {
                    let cond = lookup_param(&ram, m1, pc + 1) != 0;
                    let target = lookup_param(&ram, m2, pc + 2);
                    if cond {
                        pc = target as usize;
                    } else {
                        pc += 3;
                    }
                }
                Opcode::JumpIfFalse(m1, m2) => {
                    let cond = lookup_param(&ram, m1, pc + 1) != 0;
                    let target = lookup_param(&ram, m2, pc + 2);
                    if !cond {
                        pc = target as usize;
                    } else {
                        pc += 3;
                    }
                }
                Opcode::LessThan(m1, m2) => {
                    let a = lookup_param(&ram, m1, pc + 1);
                    let b = lookup_param(&ram, m2, pc + 2);
                    let t = ram[pc + 3] as usize;
                    ram[t] = if a < b { 1 } else { 0 };
                    pc += 4;
                }
                Opcode::Equals(m1, m2) => {
                    let a = lookup_param(&ram, m1, pc + 1);
                    let b = lookup_param(&ram, m2, pc + 2);
                    let t = ram[pc + 3] as usize;
                    ram[t] = if a == b { 1 } else { 0 };
                    pc += 4;
                }
                Opcode::Halt => break,
            }
        }

        ram
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
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),
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
        5 => Opcode::JumpIfTrue(m1, m2),
        6 => Opcode::JumpIfFalse(m1, m2),
        7 => Opcode::LessThan(m1, m2),
        8 => Opcode::Equals(m1, m2),
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

    fn exec_test(rom: Vec<i32>, phases: &[i32], expected: i32) {
        assert_eq!(amp_series(rom, phases), expected);
    }

    fn exec_test2(rom: Vec<i32>, _: &[i32], expected: i32) {
        assert_eq!(max_5amp_signal(rom), expected);
    }

    #[test]
    fn test_amp_series() {
        exec_test(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], &vec![4,3,2,1,0], 43210);
        exec_test(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], &vec![0,1,2,3,4], 54321);
        exec_test(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], &vec![1,0,4,3,2], 65210);
    }

    #[test]
    fn test_max_5amp_signal() {
        exec_test2(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], &vec![4,3,2,1,0], 43210);
        exec_test2(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0], &vec![0,1,2,3,4], 54321);
        exec_test2(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], &vec![1,0,4,3,2], 65210);     
    }
}
