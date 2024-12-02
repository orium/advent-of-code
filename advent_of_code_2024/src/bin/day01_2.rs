use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/01");

fn main() {
    let (values_a, values_b): (Vec<_>, Vec<_>) =
        INPUT.lines().map(|line| scan_fmt!(line, "{} {}", i64, i64).unwrap()).unzip();

    let b_counts: HashMap<i64, usize> = values_b.into_iter().counts();

    let r: i64 = values_a
        .into_iter()
        .map(|v| v * (b_counts.get(&v).cloned().unwrap_or_default() as i64))
        .sum();

    println!("{r}");
}
