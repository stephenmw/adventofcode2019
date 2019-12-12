use crate::intcode;
use crate::intcode::State;

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

pub fn main() {
    let program = intcode::read_program("data/day11.txt").expect("failed to read program");
    println!("Part 1: {}", count_painted_tiles(&program));
    part2(&program);
}

fn count_painted_tiles(program: &[i64]) -> usize {
    let mut tiles = HashMap::new();
    paint_tiles(program, &mut tiles, Point { x: 0, y: 0 });
    tiles.len()
}

fn part2(program: &[i64]) {
    let mut tiles = HashMap::new();
    let start = Point { x: 0, y: 0 };
    tiles.insert(start, Color::White);
    paint_tiles(program, &mut tiles, start);

    println!("{}", render_tiles(&tiles));
}

fn render_tiles(tiles: &HashMap<Point, Color>) -> String {
    let min_x = tiles.keys().map(|p| p.x).min().unwrap();
    let max_x = tiles.keys().map(|p| p.x).max().unwrap();
    let min_y = tiles.keys().map(|p| p.y).min().unwrap();
    let max_y = tiles.keys().map(|p| p.y).max().unwrap();

    if min_x < 0 || min_y < 0 {
        unimplemented!("negative x and y not implemented");
    }

    let x_len = max_x + 1;
    let y_len = max_y + 1;

    let mut data = vec![' ' as u8; (x_len * y_len) as usize];
    let white_tiles = tiles
        .iter()
        .filter(|(_, v)| **v == Color::White)
        .map(|(k, _)| k);

    for p in white_tiles {
        let i = (p.y * x_len + p.x) as usize;
        data[i] = 'X' as u8;
    }

    let rows: Vec<_> = data
        .chunks(x_len as usize)
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .collect();

    rows.join("\n")
}

fn paint_tiles(program: &[i64], tiles: &mut HashMap<Point, Color>, start: Point) {
    let mut robot = Robot::new(program);
    let mut loc = start;

    loop {
        let color = tiles.get(&loc).cloned().unwrap_or(Color::Black);

        match robot.step(&loc, color) {
            Some((new_color, new_loc)) => {
                if tiles.contains_key(&loc) || new_color == Color::White {
                    tiles.insert(loc, new_color);
                }

                loc = new_loc;
            }
            None => break,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

struct Robot {
    cpu: intcode::Computer,
    direction: Direction,
}

impl Robot {
    fn new(program: &[i64]) -> Robot {
        Robot {
            cpu: intcode::Computer::new(program),
            direction: Direction::North,
        }
    }

    // Takes current tile and color of that tile. Returns the new color of the
    // tile and the next tile to go to. It will return None when it halts.
    fn step(&mut self, loc: &Point, color: Color) -> Option<(Color, Point)> {
        if self.cpu.execute() == State::Halted {
            return None;
        }
        self.cpu.input(color as i64);

        let new_color = match self.cpu.execute() {
            State::Output(x) => x.try_into().unwrap(),
            _ => panic!("expected output"),
        };

        let turn = match self.cpu.execute() {
            State::Output(x) => x,
            _ => panic!("expected output"),
        };

        self.direction = match turn {
            0 => self.direction.left(),
            1 => self.direction.right(),
            _ => panic!("unknown turn"),
        };

        let new_loc = self.direction.step(loc);

        Some((new_color, new_loc))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Black = 0,
    White = 1,
}

impl TryFrom<i64> for Color {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Color::Black),
            1 => Ok(Color::White),
            _ => Err("unknown color"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    fn step(&self, p: &Point) -> Point {
        match self {
            Direction::North => Point { x: p.x, y: p.y - 1 },
            Direction::West => Point { x: p.x + 1, y: p.y },
            Direction::South => Point { x: p.x, y: p.y + 1 },
            Direction::East => Point { x: p.x - 1, y: p.y },
        }
    }
}
