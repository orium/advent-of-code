use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};
use itertools::{Itertools, MinMaxResult};

fn apply_rules(p: &str, rules: &HashMap<(char, char), char>) -> String {
    let mut next = String::with_capacity(2 * p.len());

    for (c, d) in p.chars().tuple_windows() {
        next.push(c);

        if let Some(&e) = rules.get(&(c, d)) {
            next.push(e);
        }
    }

    next.push(p.chars().last().unwrap());

    next
}

fn main() -> io::Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut lines = reader.lines();

    let mut p = lines.next().unwrap()?;
    let mut rules: HashMap<(char, char), char> = HashMap::new();

    while let Some(line) = lines.next() {
        let line: String = line?.parse().unwrap();

        if line.trim().is_empty() {
            continue;
        }

        rules.insert((line.chars().nth(0).unwrap(), line.chars().nth(1).unwrap()), line.chars().nth(6).unwrap());
    }

    for _ in 0..10 {
        p = apply_rules(&p, &rules);
        // println!("{}", p);
    }

    let counts: HashMap<char, usize> = p.chars().counts();
    let minmax = counts.iter().minmax_by_key(|(_, v)| **v);

    if let MinMaxResult::MinMax((min, _), (max, _)) = minmax {
        println!("{}", counts.get(max).unwrap() - counts.get(min).unwrap());
    }

    Ok(())
}
