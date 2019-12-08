use crate::intcode;
use std::cmp::max;

pub fn main() {
    let program = intcode::read_program("data/day7.txt").expect("failed to read program");
    println!("Part 1: {}", max_5amp_signal(&program));
    println!("Part 2: {}", part2(&program));
}

fn part2(program: &[i32]) -> i32 {
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

fn feedbad_signal(program: &[i32], phases: &[i32]) -> i32 {
    let mut output = 0;
    let mut amps = AmpSeries::new(program, phases);
    while !amps.done() {
        output = amps.run(output)
    }

    output
}

fn max_5amp_signal(program: &[i32]) -> i32 {
    let mut ret = 0;

    for a in 0..5 {
        for b in 0..5 {
            for c in 0..5 {
                for d in 0..5 {
                    for e in 0..5 {
                        let candidate = &vec![a, b, c, d, e];
                        if only_one_used(candidate.clone()) {
                            let mut amps = AmpSeries::new(program, &vec![a, b, c, d, e]);
                            ret = max(ret, amps.run(0));
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
    fn new(program: &[i32], phases: &[i32]) -> AmpSeries {
        AmpSeries {
            amps: phases
                .iter()
                .map(|&x| Amp::new(program.to_vec(), x))
                .collect(),
        }
    }

    fn run(&mut self, input: i32) -> i32 {
        let mut output = input;
        for amp in self.amps.iter_mut() {
            output = amp.run(output);
        }
        output
    }

    fn done(&self) -> bool {
        self.amps.last().map(|x| x.cpu.is_halted()).unwrap_or(true)
    }
}

struct Amp {
    cpu: intcode::Computer,
}

impl Amp {
    fn new(program: Vec<i32>, phase: i32) -> Amp {
        let mut amp = Amp {
            cpu: intcode::Computer::new(program),
        };
        amp.cpu.execute();
        amp.cpu.input(phase);
        amp.cpu.execute();

        amp
    }

    fn run(&mut self, input: i32) -> i32 {
        self.cpu.execute();
        self.cpu.input(input);
        let ret = match self.cpu.execute() {
            intcode::State::Output(x) => x,
            _ => panic!("expected output"),
        };

        self.cpu.execute();

        ret
    }
}

fn only_one_used(xs: Vec<i32>) -> bool {
    let mut xs = xs;
    xs.sort();
    xs[..].windows(2).all(|x| x[0] != x[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn exec_test(rom: Vec<i32>, phases: &[i32], expected: i32) {
        assert_eq!(AmpSeries::new(&rom, phases).run(0), expected);
    }

    fn exec_test2(rom: Vec<i32>, _: &[i32], expected: i32) {
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
