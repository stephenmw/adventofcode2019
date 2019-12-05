use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use num::range_step;

pub fn main() {
    let paths = read_paths("data/day3.txt").expect("failed to load paths");
    println!("Part 1: {}", distance1(&paths[0], &paths[1]).unwrap());
    println!("Part 2: {}", distance2(&paths[0], &paths[1]).unwrap());
}

fn distance1(path1: &[Move], path2: &[Move]) -> Option<i32> {
    junction(path1, path2).iter()
        .filter(|(p, _)| *p != Point{x: 0, y: 0})
        .map(|(p, _)| p.x.abs() + p.y.abs())
        .min()
}

fn distance2(path1: &[Move], path2: &[Move]) -> Option<i32> {
    junction(path1, path2).iter()
        .filter(|(p, _)| *p != Point{x: 0, y: 0})
        .map(|(_, v)| *v)
        .min()
}

fn junction(path1: &[Move], path2: &[Move]) -> Vec<(Point, i32)> {
    let path1_points = point_set(path1);
    let path2_points = point_set(path2);
    intersection(&path1_points, &path2_points)
}

fn intersection(m1: &HashMap<Point, i32>, m2: &HashMap<Point, i32>) -> Vec<(Point, i32)> {
    let s1: HashSet<_> = m1.keys().collect();
    let s2: HashSet<_> = m2.keys().collect();
    s1.intersection(&s2)
        .map(|&k| (*k, m1.get(k).unwrap() + m2.get(k).unwrap()) )
        .collect()
}

fn point_set(path: &[Move]) -> HashMap<Point, i32> {
    let mut ret = HashMap::new();
    let mut last = Point{x: 0, y: 0};
    let mut n = 0;
    for m in path {
        for point in points(last, *m) {
            last = point;
            n += 1;
            ret.entry(point).or_insert(n);
        }
    }
    ret
}

fn read_paths<P: AsRef<Path>>(filename: P) -> Result<Vec<Vec<Move>>, Box<dyn Error>> {
    let mut ret: Vec<Vec<Move>> = Vec::new();
    let raw = fs::read_to_string(filename)?;
    for line in raw.lines() {
        let path = parse_path(line)?;
        ret.push(path);
    }

    Ok(ret)
}

fn parse_path(s: &str) -> Result<Vec<Move>, Box<dyn Error>> {
    s.split_terminator(',')
            .try_fold(Vec::new(), |mut acc, x| {
                acc.push(x.parse()?);
                Ok(acc)
            })
}

fn points(start: Point, next_move: Move) -> Box<dyn Iterator<Item = Point>> {
    let l = next_move.length;
    let sx = start.x;
    let sy = start.y;
    match next_move.direction {
        Direction::Up => Box::new(((sy+1)..(sy+l+1)).map(move |y| start.with_y(y))),
        Direction::Down => Box::new(range_step(sy-1, sy-l-1, -1).map(move |y| start.with_y(y))),
        Direction::Right => Box::new(((sx+1)..(sx+l+1)).map(move |x| start.with_x(x))),
        Direction::Left => Box::new(range_step(sx-1, sx-l-1, -1).map(move |x| start.with_x(x))),
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn with_x(&self, x: i32) -> Point {
        Point{x: x, y: self.y}
    }

    fn with_y(&self, y: i32) -> Point {
        Point{x: self.x, y: y}
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(src: char) -> Result<Direction, &'static str> {
        match src {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err("failed to parse direction"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Move {
    direction: Direction,
    length: i32,
}

impl FromStr for Move {
    type Err = Box<dyn Error>;

    fn from_str(src: &str) -> Result<Move, Box<dyn Error>> {
        Ok(
            Move {
                direction: Direction::from_char(
                    src.chars()
                        .next()
                        .ok_or("failed to get direction")?
                )?,
                length: src[1..].parse()?,
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance1() {
        let tests = vec![
            ("R8,U5,L5,D3","U7,R6,D4,L4", 6),
            ("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83", 159),
            ("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 135),
        ];

        for (path1, path2, expected) in tests {
            let p1 = parse_path(path1).unwrap();
            let p2 = parse_path(path2).unwrap();
            let result = distance1(&p1, &p2).unwrap();
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_distance2() {
        let tests = vec![
            ("R8,U5,L5,D3","U7,R6,D4,L4", 30),
            ("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83", 610),
            ("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7", 410),
        ];

        for (path1, path2, expected) in tests {
            let p1 = parse_path(path1).unwrap();
            let p2 = parse_path(path2).unwrap();
            let result = distance2(&p1, &p2).unwrap();
            assert_eq!(result, expected);
        }
    }
}
