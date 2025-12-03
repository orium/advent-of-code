use itertools::Itertools;
use ndarray::Array2;
use std::cmp::max;

const INPUT: &str = include_str!("../../inputs/03");

fn max_jolts(bank: &[u32], memo: &mut Array2<Option<u64>>, i: usize, count: usize) -> u64 {
    if count >= 12 || i >= bank.len() {
        return 0;
    }

    if let Some(r) = memo[(i, count)] {
        return r;
    }

    let r = max(
        max_jolts(bank, memo, i + 1, count),
        bank[i] as u64 + 10 * max_jolts(bank, memo, i + 1, count + 1),
    );

    memo[(i, count)] = Some(r);

    r
}

fn main() {
    let sum: u64 = INPUT
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).rev().collect_vec())
        .map(|bank| {
            let mut memo: Array2<Option<u64>> = Array2::from_elem((128, 12), None);

            max_jolts(&bank, &mut memo, 0, 0)
        })
        .sum();

    println!("{sum}");
}
