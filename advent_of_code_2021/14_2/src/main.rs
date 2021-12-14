use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};
use itertools::{Itertools, MinMaxResult};

fn apply_rules(p: HashMap<(char, char), u64> , rules: &HashMap<(char, char), char>) -> HashMap<(char, char), u64> {
    let mut next = HashMap::with_capacity(2 * p.len());

    for ((c, d), count) in p.into_iter() {
        match rules.get(&(c, d)) {
            None => {
                *next.entry((c, d)).or_default() += count;
            }
            Some(&m) => {
                *next.entry((c, m)).or_default() += count;
                *next.entry((m, d)).or_default() += count;
            }
        }
    }

    next
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut lines = reader.lines();

    let mut l = {
        let l = lines.next().unwrap()?;
        format!("|{}|", l)
    };

    let mut p: HashMap<(char, char), u64> = HashMap::new();

    for (c, d) in l.chars().tuple_windows() {
        *p.entry((c, d)).or_default() += 1;
    }

    let mut rules: HashMap<(char, char), char> = HashMap::new();

    while let Some(line) = lines.next() {
        let line: String = line?.parse().unwrap();

        if line.trim().is_empty() {
            continue;
        }

        rules.insert((line.chars().nth(0).unwrap(), line.chars().nth(1).unwrap()), line.chars().nth(6).unwrap());
    }

    for _ in 0..40 {
        p = apply_rules(p, &rules);
    }

    let mut counts: HashMap<char, u64> = HashMap::new();

    for ((c, _), count) in p.into_iter() {
        if c != '|' {
            *counts.entry(c).or_default() += count;
        }
    }

    let minmax = counts.iter().minmax_by_key(|(_, v)| **v);

    if let MinMaxResult::MinMax((min, _), (max, _)) = minmax {
        println!("{}", counts.get(max).unwrap() - counts.get(min).unwrap());
    }

    Ok(())
}
