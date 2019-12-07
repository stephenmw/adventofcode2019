use crate::intcode;

pub fn main() {
    let program = intcode::read_program("data/day2.txt").expect("failed to read data");
    println!("Part 1: {}", part1(&program));
    println!("Part 2: {}", part2(&program).unwrap());
}

fn part1(program: &[i32]) -> i32 {
    execute_with_params(&program, 12, 2)
}

fn part2(input: &[i32]) -> Option<i32> {
    for a in 0..input.len() as i32 {
        for b in 0..input.len() as i32 {
            let ret = execute_with_params(&input, a, b);
            if ret == 19690720 {
                return Some(a * 100 + b);
            }
        }
    }

    None
}

fn execute_with_params(input: &[i32], a: i32, b: i32) -> i32 {
    let mut mem = input.to_vec();
    mem[1] = a;
    mem[2] = b;
    let mut cpu = intcode::Computer::new(mem);
    cpu.execute();
    cpu.ram[0]
}
