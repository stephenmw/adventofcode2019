use crate::intcode;
use std::cmp::max;

pub fn main() {
    let program = intcode::read_program("data/day07.txt").expect("failed to read program");
    println!("Part 1: {}", max_5amp_signal(&program));
    println!("Part 2: {}", part2(&program));
}

fn part2(program: &[i64]) -> i64 {
    let mut ret = 0;

    for a in 5..10 {
        for b in 5..10 {
            for c in 5..10 {
                for d in 5..10 {
                    for e in 5..10 {
                        let candidate = &vec![a, b, c, d, e];
                        if only_one_used(candidate.clone()) {
                            ret = max(ret, feedbad_signal(program, &vec![a, b, c, d, e]));
                        }
                    }
                }
            }
        }
    }

    ret
}

fn feedbad_signal(program: &[i64], phases: &[i64]) -> i64 {
    let mut output = 0;
    let mut amps = AmpSeries::new(program, phases);
    loop {
        output = match amps.run(output) {
            Some(x) => x,
            None => break,
        }
    }

    output
}

fn max_5amp_signal(program: &[i64]) -> i64 {
    let mut ret = 0;

    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        let candidate = &vec![a, b, c, d, e];
                        if only_one_used(candidate.clone()) {
                            let mut amps = AmpSeries::new(program, &vec![a, b, c, d, e]);
                            ret = max(ret, amps.run(0).unwrap());
                        }
                    }
                }
            }
        }
    }

    ret
}

struct AmpSeries {
    amps: Vec<Amp>,
}

impl AmpSeries {
    fn new(program: &[i64], phases: &[i64]) -> AmpSeries {
        AmpSeries {
            amps: phases.iter().map(|&x| Amp::new(program, x)).collect(),
        }
    }

    fn run(&mut self, input: i64) -> Option<i64> {
        let mut output = input;
        for amp in self.amps.iter_mut() {
            output = match amp.run(output) {
                Some(x) => x,
                None => return None,
            };
        }
        Some(output)
    }
}

struct Amp {
    cpu: intcode::Computer,
}

impl Amp {
    fn new(program: &[i64], phase: i64) -> Amp {
        let mut amp = Amp {
            cpu: intcode::Computer::new(program),
        };
        amp.cpu.execute();
        amp.cpu.input(phase);

        amp
    }

    fn run(&mut self, input: i64) -> Option<i64> {
        if self.cpu.execute() == intcode::State::Halted {
            return None;
        }
        self.cpu.input(input);

        let ret = match self.cpu.execute() {
            intcode::State::Output(x) => x,
            intcode::State::Halted => return None,
            _ => panic!("expected output or halt"),
        };

        Some(ret)
    }
}

fn only_one_used(xs: Vec<i64>) -> bool {
    let mut xs = xs;
    xs.sort();
    xs[..].windows(2).all(|x| x[0] != x[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exec_test(rom: Vec<i64>, phases: &[i64], expected: i64) {
        assert_eq!(AmpSeries::new(&rom, phases).run(0), Some(expected));
    }

    fn exec_test2(rom: Vec<i64>, _: &[i64], expected: i64) {
        assert_eq!(max_5amp_signal(&rom), expected);
    }

    #[test]
    fn test_amp_series() {
        exec_test(
            vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            &vec![4, 3, 2, 1, 0],
            43210,
        );
        exec_test(
            vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ],
            &vec![0, 1, 2, 3, 4],
            54321,
        );
        exec_test(
            vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ],
            &vec![1, 0, 4, 3, 2],
            65210,
        );
    }

    #[test]
    fn test_max_5amp_signal() {
        exec_test2(
            vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ],
            &vec![4, 3, 2, 1, 0],
            43210,
        );
        exec_test2(
            vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ],
            &vec![0, 1, 2, 3, 4],
            54321,
        );
        exec_test2(
            vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ],
            &vec![1, 0, 4, 3, 2],
            65210,
        );
    }
}
