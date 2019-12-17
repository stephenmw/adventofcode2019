use std::fs;
use std::io;
use std::iter;
use std::path::Path;

const BASE_PATTERN: [i8; 4] = [0, 1, 0, -1];

pub fn main() {
    let input = read_input("data/day16.txt").expect("failed to read input");
    let input10000: Vec<_> = input
        .iter()
        .copied()
        .cycle()
        .take(input.len() * 10000)
        .collect();

    println!("Part 1: {}", render_output(&fft(&input, 100)[..8]));
    println!("Part 2: {}", part2(&input10000));
}

fn part2(input: &[u8]) -> String {
    let offset: usize = render_output(&input[..7]).parse().unwrap();
    let message = input[offset..].to_vec();

    let ret = (0..100).fold(message, |message, _| {
        let mut new: Vec<_> = message
            .iter()
            .rev()
            .scan(0, |acc, &next| {
                *acc += next as i64;
                Some((*acc % 10) as u8)
            })
            .collect();
        new.reverse();
        new
    });

    render_output(&ret[..8])
}

fn fft(input: &[u8], phases: usize) -> Vec<u8> {
    (0..phases).fold(input.to_vec(), |cur, _| phase(&cur))
}

fn phase(input: &[u8]) -> Vec<u8> {
    (0..input.len())
        .map(|pos| phase_digit(input, pos + 1))
        .collect()
}

fn phase_digit(input: &[u8], pos: usize) -> u8 {
    let p = pattern(pos);
    let n: i64 = input
        .iter()
        .zip(p)
        .map(|(&d, pat)| d as i64 * pat as i64)
        .sum();
    (n.abs() % 10) as u8
}

fn pattern(pos: usize) -> impl Iterator<Item = i8> {
    BASE_PATTERN
        .iter()
        .cloned()
        .flat_map(move |x| iter::repeat(x).take(pos))
        .cycle()
        .skip(1)
}

fn read_input<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let raw = fs::read_to_string(path)?;
    Ok(parse_input(&raw))
}

fn parse_input(s: &str) -> Vec<u8> {
    s.trim()
        .bytes()
        .filter(|&x| x >= '0' as u8 && x <= '9' as u8)
        .map(|x| x - '0' as u8)
        .collect()
}

fn render_output(xs: &[u8]) -> String {
    String::from_utf8(xs.iter().map(|&x| x + '0' as u8).collect()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase() {
        fn exec(input: &str, output: &str) {
            let data = parse_input(input);
            let res = render_output(&phase(&data));
            assert_eq!(&res, output);
        }

        exec("12345678", "48226158");
        exec("48226158", "34040438");
        exec("34040438", "03415518");
        exec("03415518", "01029498");
    }
}
