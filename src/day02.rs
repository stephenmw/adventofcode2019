use crate::intcode;

pub fn main() {
    let program = intcode::read_program("data/day02.txt").expect("failed to read data");
    println!("Part 1: {}", part1(&program));
    println!("Part 2: {}", part2(&program).unwrap());
}

fn part1(program: &[i64]) -> i64 {
    execute_with_params(&program, 12, 2)
}

fn part2(input: &[i64]) -> Option<i64> {
    for a in 0..input.len() as i64 {
        for b in 0..input.len() as i64 {
            let ret = execute_with_params(&input, a, b);
            if ret == 19690720 {
                return Some(a * 100 + b);
            }
        }
    }

    None
}

fn execute_with_params(input: &[i64], a: i64, b: i64) -> i64 {
    let mut cpu = intcode::Computer::new(input);
    cpu.ram[1] = a;
    cpu.ram[2] = b;
    cpu.execute();
    cpu.ram[0]
}
