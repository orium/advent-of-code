use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/01");

fn main() {
    let (values_a, values_b): (Vec<_>, Vec<_>) =
        INPUT.lines().map(|line| scan_fmt!(line, "{} {}", i64, i64).unwrap()).unzip();

    let r: i64 =
        values_a.iter().sorted().zip(values_b.iter().sorted()).map(|(a, b)| (a - b).abs()).sum();

    println!("{r}");
}
