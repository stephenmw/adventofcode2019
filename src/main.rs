mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    let day = 5;
    match day {
        1 => day1::main(),
        2 => day2::main(),
        3 => day3::main(),
        4 => day4::main(),
        5 => day5::main(),
        _ => panic!("bad day"),
    };
}
