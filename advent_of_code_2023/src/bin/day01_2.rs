const INPUT: &str = include_str!("../../inputs/01");

const NUMS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn get_cal(s: &str) -> Option<u32> {
    NUMS.iter().find_map(|(ns, n)| match s.starts_with(ns) {
        true => Some(*n),
        false => s.chars().next().and_then(|c| c.to_digit(10)),
    })
}

fn main() {
    let v: u64 = INPUT
        .lines()
        .map(|line| {
            let first = (0..line.len()).find_map(|i| get_cal(&line[i..])).unwrap() as u64;
            let last = (0..line.len()).rev().find_map(|i| get_cal(&line[i..])).unwrap() as u64;
            first * 10 + last
        })
        .sum();

    println!("{}", v);
}
