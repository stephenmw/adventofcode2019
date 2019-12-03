use std::error::Error;
use std::fs;
use std::path::Path;

pub fn main() {
    let input = read_integers("data/day2.txt").expect("failed to read data");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input).unwrap());
}

fn part1(input: &[usize]) -> usize {
    execute_with_params(&input, 12, 2)
}

fn part2(input: &[usize]) -> Option<usize> {
    for a in 0..input.len() {
        for b in 0..input.len() {
            let ret = execute_with_params(&input, a, b);
            if ret == 19690720 {
                return Some(a * 100 + b);
            }
        }
    }

    None
}

fn execute_with_params(input: &[usize], a: usize, b: usize) -> usize {
    let mut mem = input.to_vec();
    mem[1] = a;
    mem[2] = b;
    execute_program(&mut mem);
    mem[0]
}

fn execute_program(mem: &mut [usize]) {
    let mut p = 0;
    loop {
        match mem[p] {
            1 => {
                let t = mem[p + 3];
                mem[t] = mem[mem[p + 1]] + mem[mem[p + 2]];
            }
            2 => {
                let t = mem[p + 3];
                mem[t] = mem[mem[p + 1]] * mem[mem[p + 2]];
            }
            99 => break,
            _ => panic!("bad program"),
        }

        p += 4;
    }
}

fn read_integers<P: AsRef<Path>>(path: P) -> Result<Vec<usize>, Box<dyn Error>> {
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

    fn exec_test(input: Vec<usize>, expected: Vec<usize>) {
        let mut mem = input.clone();
        execute_program(&mut mem);
        assert_eq!(mem, expected);
    }

    #[test]
    fn test_execute_program() {
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
