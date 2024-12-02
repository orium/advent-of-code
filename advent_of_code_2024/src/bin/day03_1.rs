use itertools::Itertools;
use regex::Regex;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/03");

fn main() {
    let mul_regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let r: i64 = INPUT
        .lines()
        .flat_map(|line| mul_regex.captures_iter(line))
        .flat_map(|captures| captures.iter().collect_vec().into_iter())
        .flatten()
        .map(|instruction| scan_fmt!(instruction.as_str(), "mul({},{})", i64, i64).unwrap())
        .map(|(a, b)| a * b)
        .sum();

    println!("{r}");
}
