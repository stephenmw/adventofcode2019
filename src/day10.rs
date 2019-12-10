use std::collections::HashSet;
use std::fs;
use std::io;
use std::path::Path;


pub fn main() {
    let astroids = read_map("data/day10.txt").expect("failed to read map");
    println!("Part 1: {}", part1(&astroids).unwrap());
}

fn read_map<P: AsRef<Path>>(path: P) -> io::Result<Vec<(usize, usize)>> {
    let raw = fs::read_to_string(path)?;

    let points = raw.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.bytes().enumerate().map(move |(x, b)| (x, y, b)));

    let astroid_locations = points
        .filter(|(_, _, b)| *b == '#' as u8)
        .map(|(x, y, _)| (x, y))
        .collect();

    Ok(astroid_locations)
}

fn part1(astroids: &[(usize, usize)]) -> Option<usize> {
    astroids.iter()
        .map(|&a| observed(a, astroids))
        .max()
}

fn observed(candidate: (usize, usize), astroids: &[(usize, usize)]) -> usize {
    let others = astroids.iter().filter(|&&x| x != candidate);
    let directions = others
        .map(|&(x, y)| {
            let delta_x = x as i64 - candidate.0 as i64;
            let delta_y = y as i64 - candidate.1 as i64;
            (delta_x, delta_y)
        });
    let simplified_directions: HashSet<_> = directions.map(|(dx, dy)| {
        let factor = gcd(dx.abs() as usize, dy.abs() as usize) as i64;
        (dx/factor, dy/factor)
    }).collect();

    simplified_directions.len()
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    loop {
        if a == 0 {
            return b;
        }

        if b == 0 {
            return a;
        }

        if a < b {
            b -= a;
        } else {
            a -= b;
        }
    }
}