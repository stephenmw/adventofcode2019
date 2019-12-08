use std::error::Error;
use std::fs;
use std::path::Path;

pub fn main() {
    let data = read_integers("data/day1.txt").expect("failed to read data");
    let ans1: i32 = data.iter().map(|&x| fuel_1(x)).sum();
    let ans2: i32 = data.iter().map(|&x| fuel_2(x)).sum();

    println!("Part 1: {}", ans1);
    println!("Part 2: {}", ans2);
}

fn fuel_1(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_2(mass: i32) -> i32 {
    let mut fuel = fuel_1(mass);
    let mut cur = fuel_1(fuel);
    while cur > 0 {
        fuel += cur;
        cur = fuel_1(cur);
    }

    fuel
}

pub fn read_integers<P: AsRef<Path>>(path: P) -> Result<Vec<i32>, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    raw.split_ascii_whitespace()
        .try_fold(Vec::new(), |mut acc, x| {
            acc.push(x.parse()?);
            Ok(acc)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_1() {
        assert_eq!(fuel_1(12), 2);
        assert_eq!(fuel_1(14), 2);
        assert_eq!(fuel_1(1969), 654);
        assert_eq!(fuel_1(100756), 33583);
    }

    #[test]
    fn test_fuel_2() {
        assert_eq!(fuel_2(12), 2);
        assert_eq!(fuel_2(1969), 966);
        assert_eq!(fuel_2(100756), 50346);
    }
}
