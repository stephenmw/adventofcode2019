use crate::intcode::{self, State};

use std::convert::{TryFrom, TryInto};
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::max;

pub fn main() {
    let program = intcode::read_program("data/day15.txt").expect("failed to read program");
    let mut droid = Droid::new(&program);
    let map = explore(&mut droid);
            println!("final loc: {:?}", droid.location);


    render_map(&map);

    println!("Part 1: {}", path_length(&map, droid.location).unwrap());
}

fn render_map(m: &HashMap<Point, Status>) {
    let min_x = m.keys().map(|p| p.x).min().unwrap();
    let max_x = m.keys().map(|p| p.x).max().unwrap();
    let min_y = m.keys().map(|p| p.y).min().unwrap();
    let max_y = m.keys().map(|p| p.y).max().unwrap();

    let len_x = (max_x - min_x + 1) as usize;
    let len_y = (max_y - min_y + 1) as usize;

    let mut data = vec![' ' as u8; len_x * len_y];

    let point_to_index = |point: &Point| {
        let adj_x = point.x - min_x;
        let adj_y = point.y - min_y;
        let i = adj_y * len_x as i64 + adj_x;
        i as usize
    };

    for (point, status) in m.iter() {
        data[point_to_index(point)] = match status {
            Status::Wall => 'X' as u8,
            Status::Empty => '.' as u8,
            Status::Oxygen => 'O' as u8,
        };
    }

    data[point_to_index(&Point{x: 0, y: 0})] = 'S' as u8;

    data.chunks(len_x)
        .for_each(|line| println!("{}", String::from_utf8(line.to_vec()).unwrap()));
}

fn path_length(m: &HashMap<Point, Status>, start: Point) -> Option<usize> {
    let mut frontier = VecDeque::new();
    let mut seen = HashSet::new();
    frontier.push_back((start, 0));
    seen.insert(start);

    while let Some((loc, count)) = frontier.pop_front() {
        let status = m.get(&loc).cloned().unwrap_or(Status::Wall);
        match status {
            Status::Empty => {
                let children = Direction::iterator()
                    .map(|dir| dir.step(&loc));

                for child in children {
                    if !seen.contains(&child) {
                        frontier.push_back((child, count + 1));
                        seen.insert(child);
                    }
                }
            },
            Status::Wall => (), // no-op
            Status::Oxygen => return Some(count),
        }
    }

    None
}

fn explore(droid: &mut Droid) -> HashMap<Point, Status> {
    let mut m = HashMap::new();
    // starting location is by definition empty (the droid is there).
    m.insert(droid.location, Status::Empty);

    let mut backtrack_stack = Vec::new();

    'outer: loop {
        for dir in Direction::iterator() {
            if *dir == Direction::West && droid.location == (Point{x: 0, y: 0}) {
                println!("there");
            }
            if !m.contains_key(&dir.step(&droid.location)) {
                let (status, point) = droid.step(*dir);
                if point == (Point{x: -1, y: 0}) {
                    println!("here");
                }
                m.insert(point, status);
                if status == Status::Empty {
                    backtrack_stack.push(dir.opposite());
                }
                continue 'outer;
            }
        }

        match backtrack_stack.pop() {
            Some(dir) => droid.step(dir),
            None => break,
        };
    }

    m
}

struct Droid {
    cpu: intcode::Computer,
    location: Point,
}

impl Droid {
    fn new(program: &[i64]) -> Droid {
        Droid {
            cpu: intcode::Computer::new(program),
            location: Point{x: 0, y: 0},
        }
    }

    fn step(&mut self, dir: Direction) -> (Status, Point) {
        self.cpu.execute();
        self.cpu.input(dir as i64);
        let output = match self.cpu.execute() {
            State::Output(x) => x,
            _ => panic!("unexpected intcode state"),
        };

        let status: Status = output.try_into().unwrap();
        let point = dir.step(&self.location);

        match status {
            Status::Empty => self.location = point,
            Status::Oxygen => {
                self.location = point;
                self.step(dir.opposite());
            }, // undo move
            Status:: Wall => (), // no-op
        };
        if status == Status::Empty {
            self.location = point;
        }

        (status, point)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Direction {
    fn step(&self, p: &Point) -> Point {
        match self {
            Direction::North => Point { x: p.x, y: p.y - 1 },
            Direction::South => Point { x: p.x, y: p.y + 1 },
            Direction::West => Point { x: p.x + 1, y: p.y },
            Direction::East => Point { x: p.x - 1, y: p.y },
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }

    pub fn iterator() -> impl Iterator<Item=&'static Direction> {
        static DIRECTIONS: [Direction;  4] = [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        DIRECTIONS.into_iter()
    }

}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Status {
    Wall = 0,
    Empty = 1,
    Oxygen = 2,
}

impl TryFrom<i64> for Status {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Status::Wall),
            1 => Ok(Status::Empty),
            2 => Ok(Status::Oxygen),
            _ => Err("unknown status"),
        }
    }
}
