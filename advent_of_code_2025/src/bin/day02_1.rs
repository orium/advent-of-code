use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/02");

fn is_invalid(id: u64) -> bool {
    let divisor = 10_u64.pow(id.ilog10().div_ceil(2));

    id / divisor == id % divisor
}

fn main() {
    let input = INPUT.lines().join("");
    let input =
        input.split(',').map(|s| scan_fmt!(s, "{}-{}", u64, u64).unwrap()).map(|(s, e)| s..=e);

    let sum: u64 = input.flat_map(|range| range.filter(|id| is_invalid(*id))).sum();

    println!("{sum}");
}
