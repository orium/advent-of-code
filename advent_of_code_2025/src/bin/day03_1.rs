use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/03");

fn max_jolts(bank: &[u32]) -> u32 {
    bank.iter().combinations(2).map(|v| v[0] * 10 + v[1]).max().unwrap_or_default()
}

fn main() {
    let sum: u32 = INPUT
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect_vec())
        .map(|bank| max_jolts(&bank))
        .sum();

    println!("{sum}");
}
