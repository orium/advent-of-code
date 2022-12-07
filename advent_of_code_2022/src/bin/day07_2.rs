use itertools::Itertools;
use scan_fmt::scan_fmt;
use std::collections::{HashMap, HashSet};
use std::iter;

const INPUT: &str = include_str!("../../inputs/07");

fn main() {
    let mut file_sizes: HashMap<String, usize> = HashMap::new();
    let mut working_dir: String = "/".to_owned();
    let mut dirs: HashSet<String> = HashSet::new();

    dirs.insert(working_dir.clone());

    for line in INPUT.lines() {
        if let Ok(dir) = scan_fmt!(line, "$ cd {}", String) {
            working_dir = match dir.as_str() {
                "/" => "/".to_owned(),
                ".." => working_dir
                    .split('/')
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .rev()
                    .skip(2)
                    .rev()
                    .chain(iter::once(""))
                    .join("/"),
                d => format!("{}{}/", working_dir, d),
            };

            if !dirs.contains(&working_dir) {
                dirs.insert(working_dir.clone());
            }
        }

        if let Ok((size, filename)) = scan_fmt!(line, "{} {}", usize, String) {
            println!("{}{} {}", working_dir, filename, size);
            file_sizes.insert(format!("{}{}", working_dir, filename), size);
        }
    }

    let total: usize = file_sizes.values().sum();
    let unused: usize = 70_000_000 - total;
    let min_needed: usize = 30_000_000 - unused;

    let mut best: usize = 100_000_000;

    println!("min_needed: {}", min_needed);

    for dir in dirs {
        let size: usize =
            file_sizes.iter().filter(|(k, _)| k.starts_with(&dir)).map(|(_, v)| v).sum();

        if size < best && size > min_needed {
            best = size;
        }
    }

    println!("{}", best);
}
