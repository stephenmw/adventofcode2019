pub fn main() {
    println!("Part 1: {}", num_password_options(359282, 820401, &validate1));
    println!("Part 2: {}", num_password_options(359282, 820401, &validate2));
}

fn num_password_options(min: u32, max: u32, filter: &dyn Fn(u32) -> bool) -> u32 {
    (min..=max).filter(|&x| filter(x)).count() as u32
}

fn validate1(n: u32) -> bool {
    let bcd: Vec<u8> = n.to_string().bytes().map(|c| c - '0' as u8).collect();
    let adj_digits = bcd.windows(2).any(|x| x[0] == x[1]);
    let monotonic = bcd.windows(2).all(|x| x[0] <= x[1]);
    adj_digits && monotonic
}

fn validate2(n: u32) -> bool {
    let bcd: Vec<u8> = n.to_string().bytes().map(|c| c - '0' as u8).collect();
    let adj_digits = single_adj_digit(&bcd);
    let monotonic = bcd.windows(2).all(|x| x[0] <= x[1]);
    adj_digits && monotonic
}

fn single_adj_digit(bcd: &[u8]) -> bool {
    let mut freq = [0; 10];
    bcd.iter().for_each(|&x| freq[x as usize] += 1);
    freq.iter().any(|&x| x == 2)
}