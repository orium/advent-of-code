use std::collections::BTreeSet;

const INPUT: &str = include_str!("../../inputs/03");

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

fn main() {
    let mut total: u32 = 0;

    for line in INPUT.lines() {
        let (a, b) = line.split_at(line.len()/2);
        let (a, b) : (BTreeSet<char>, BTreeSet<char>) = (a.chars().collect(), b.chars().collect());

        total += a.intersection(&b).copied().map(priority).sum::<u32>();
    }

    println!("{}", total);
}
