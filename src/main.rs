mod day1;
mod day2;
mod day3;

fn main() {
    let day = 3;
    match day {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        _ => panic!("bad day"),
    };
}
