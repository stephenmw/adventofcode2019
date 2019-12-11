use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

pub fn main() {
    let astroids = read_map("data/day10.txt").expect("failed to read map");
    let (base, relative) = most_observed(&astroids).unwrap();

    println!("Part 1: {}", relative.len());

    let order = firing_order(&base, relative);
    let target = order[199];

    println!("Part 2: {}", target.x * 100 + target.y);
}

fn firing_order(base: &Point, relative: HashMap<Direction, Vec<Point>>) -> Vec<Point> {
    fn distance(p1: &Point, p2: &Point) -> i64 {
        (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
    }

    let mut relative = relative;
    relative
        .values_mut()
        .for_each(|vs| vs.sort_by_key(|v| distance(v, base)));

    let mut relative_positions: Vec<_> = relative
        .iter()
        .flat_map(|(d, ps)| ps.iter().enumerate().map(move |(i, p)| (i, *d, *p)))
        .collect();

    relative_positions.sort_by(|(i1, d1, _), (i2, d2, _)| {
        let i_order = i1.cmp(i2);
        match i_order {
            Ordering::Equal => d1.cmp(d2),
            _ => i_order,
        }
    });

    relative_positions.into_iter().map(|(_, _, p)| p).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Quadrant {
    Q1 = 1,
    Q2 = 2,
    Q3 = 3,
    Q4 = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Direction {
    dx: i64,
    dy: i64,
}

impl Direction {
    fn from_points(src: &Point, dst: &Point) -> Direction {
        Direction {
            dx: dst.x - src.x,
            // Original origin is in the upper left so we need to flip
            // the y axis.
            dy: src.y - dst.y,
        }
    }

    fn simplified(&self) -> Direction {
        let gcf = gcd(self.dx.abs() as usize, self.dy.abs() as usize) as i64;
        Direction {
            dx: self.dx / gcf,
            dy: self.dy / gcf,
        }
    }

    fn quadrant(&self) -> Quadrant {
        match (!self.dx.is_negative(), !self.dy.is_negative()) {
            (true, true) => Quadrant::Q1,
            (true, false) => Quadrant::Q2,
            (false, false) => Quadrant::Q3,
            (false, true) => Quadrant::Q4,
        }
    }
}

impl Ord for Direction {
    fn cmp(&self, other: &Self) -> Ordering {
        // division of non-NaN cannot produce NaN.
        self.partial_cmp(other).unwrap()
    }
}

// Orders directions clockwise starting at 0 on a clock.
impl PartialOrd for Direction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.quadrant().cmp(&other.quadrant()) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => {
                let slope1 = self.dy.abs() as f64 / self.dx.abs() as f64;
                let slope2 = other.dy.abs() as f64 / other.dx.abs() as f64;
                let ret = slope1.partial_cmp(&slope2);
                match self.quadrant() {
                    Quadrant::Q1 | Quadrant::Q3 => ret.map(|o| o.reverse()),
                    Quadrant::Q2 | Quadrant::Q4 => ret,
                }
            }
        }
    }
}

fn read_map<P: AsRef<Path>>(path: P) -> io::Result<Vec<Point>> {
    let raw = fs::read_to_string(path)?;

    let points = raw
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.bytes().enumerate().map(move |(x, b)| (x, y, b)));

    let astroid_locations = points
        .filter(|(_, _, b)| *b == '#' as u8)
        .map(|(x, y, _)| Point::new(x as i64, y as i64))
        .collect();

    Ok(astroid_locations)
}

fn most_observed(astroids: &[Point]) -> Option<(Point, HashMap<Direction, Vec<Point>>)> {
    astroids
        .iter()
        .map(|a| (*a, observed(a, astroids)))
        .max_by_key(|(_, x)| x.len())
}

fn observed(candidate: &Point, astroids: &[Point]) -> HashMap<Direction, Vec<Point>> {
    let others = astroids.iter().filter(|&x| x != candidate);

    let mut m = HashMap::new();
    for other in others {
        let direction = Direction::from_points(candidate, other);
        m.entry(direction.simplified())
            .or_insert(Vec::new())
            .push(*other);
    }

    m
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
