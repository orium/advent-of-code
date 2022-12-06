use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/06");

fn main() {
    let chars = INPUT.chars().collect_vec();

    let index = chars
        .windows(14)
        .enumerate()
        .find(|(_, w)| w.iter().collect::<HashSet<_>>().len() == 14)
        .map(|(i, _)| i)
        .unwrap();

    println!("{}", index + 14);
}
