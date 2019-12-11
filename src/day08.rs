use std::fs;
use std::io;
use std::path::Path;

const HEIGHT: usize = 6;
const WIDTH: usize = 25;

pub fn main() {
    let image = read_image("data/day08.txt").expect("failed to read image");
    println!("Part 1: {}", part1(&image));
    println!("{}", part2(&image));
}

fn part1(image: &[u8]) -> usize {
    let layer = image
        .chunks(HEIGHT * WIDTH)
        .min_by_key(|x| x.iter().filter(|&d| *d == 0).count())
        .unwrap();

    let ones = layer.iter().filter(|&d| *d == 1).count();
    let twos = layer.iter().filter(|&d| *d == 2).count();
    ones * twos
}

fn part2(image: &[u8]) -> String {
    let layers: Vec<_> = image.chunks(HEIGHT * WIDTH).collect();
    let final_image: Vec<_> = (0..(HEIGHT * WIDTH))
        .map(|pos| pixel(&layers, pos))
        .collect();
    render(&final_image)
}

fn render(image: &[u8]) -> String {
    let text: Vec<_> = image
        .iter()
        .map(|&x| if x == 1 { 'x' as u8 } else { ' ' as u8 })
        .collect();
    let rows: Vec<_> = text
        .chunks(WIDTH)
        .map(|x| String::from_utf8(x.to_vec()).unwrap())
        .collect();
    rows.join("\n")
}

fn pixel(layers: &[&[u8]], pos: usize) -> u8 {
    for layer in layers {
        if layer[pos] != 2 {
            return layer[pos];
        }
    }
    2
}

fn read_image<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let raw = fs::read_to_string(path)?;
    Ok(raw
        .bytes()
        .filter(|&x| x >= '0' as u8 && x <= '9' as u8)
        .map(|x| x - '0' as u8)
        .collect())
}
