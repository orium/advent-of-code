const INPUT: &str = include_str!("../../inputs/01");

fn main() {
    let v: u64 = INPUT
        .lines()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap() as u64;
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap() as u64;

            first * 10 + last
        })
        .sum();

    println!("{v}");
}
