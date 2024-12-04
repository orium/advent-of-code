use itertools::Itertools;
use regex::Regex;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/03");

fn main() {
    let mul_regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let (_, r) = INPUT
        .lines()
        .flat_map(|line| mul_regex.captures_iter(line))
        .flat_map(|captures| captures.iter().collect_vec().into_iter())
        .flatten()
        .fold((true, 0), |(enabled, sum), instruction| match instruction.as_str() {
            "do()" => (true, sum),
            "don't()" => (false, sum),
            _ if enabled => {
                let (a, b) = scan_fmt!(instruction.as_str(), "mul({},{})", i64, i64).unwrap();
                (true, sum + a * b)
            }
            _ => (enabled, sum),
        });

    println!("{r}");
}
