use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::path::Path;

pub fn main() {
    let data = read_data("data/day6.txt").unwrap();
    println!("Part 1: {}", count_orbits(&data));
    println!("Part 2: {}", quickest_path_length(&data));
}

fn count_orbits(data: &[(String, String)]) -> usize {
    let mut m: HashMap<&str, Vec<&str>> = HashMap::new();
    data.iter()
        .for_each(|(k, v)| m.entry(k).or_insert(Vec::new()).push(v));

    let mut ret = 0;
    let mut stack = vec![("COM", 0)];

    while let Some((obj, count)) = stack.pop() {
        ret += count;
        m.get(obj)
            .unwrap_or(&Vec::new())
            .iter()
            .for_each(|x| stack.push((x, count + 1)));
    }

    ret
}

fn quickest_path_length(data: &[(String, String)]) -> usize {
    let mut m: HashMap<&str, Vec<&str>> = HashMap::new();
    data.iter().for_each(|(k, v)| {
        m.entry(k).or_insert(Vec::new()).push(v);
        m.entry(v).or_insert(Vec::new()).push(k);
    });

    let start: &str = m.get("YOU").and_then(|x| x.first()).unwrap();
    let end: &str = m.get("SAN").and_then(|x| x.first()).unwrap();

    let mut frontier = VecDeque::new();
    let mut seen = HashSet::new();
    frontier.push_back((start, 0));
    seen.insert(start);

    while let Some((obj, count)) = frontier.pop_front() {
        if obj == end {
            return count;
        }

        for child in m.get(obj).unwrap() {
            if seen.contains(child) {
                continue;
            }
            frontier.push_back((child, count + 1));
            seen.insert(child);
        }
    }

    unreachable!("no santa?");
}

fn read_data<P: AsRef<Path>>(filename: P) -> io::Result<Vec<(String, String)>> {
    let raw = fs::read_to_string(filename)?;
    Ok(raw
        .lines()
        .map(|s| {
            let mut iter = s.split_terminator(")");
            let parent = iter.next().unwrap();
            let child = iter.next().unwrap();
            (parent.to_string(), child.to_string())
        })
        .collect())
}
