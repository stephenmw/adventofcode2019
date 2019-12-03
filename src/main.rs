mod day1;
mod day2;

fn main() {
    let day = 1;
    match day {
        1 => day1::main(),
        2 => day2::main(),
        _ => panic!("bad day"),
    };
}
