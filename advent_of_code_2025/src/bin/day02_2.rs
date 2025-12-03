use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/02");

fn is_invalid(id: u64) -> bool {
    let id: Vec<char> = id.to_string().chars().collect_vec();

    (2..=id.len()).filter(|d| id.len().is_multiple_of(*d)).any(|div| {
        let pattern_len = id.len() / div;
        let pattern = &id[0..pattern_len];
        let repeated: Vec<char> =
            std::iter::repeat_n(pattern.iter().copied(), div).flatten().collect_vec();

        repeated == id
    })
}

fn main() {
    let input = INPUT.lines().join("");
    let input =
        input.split(',').map(|s| scan_fmt!(s, "{}-{}", u64, u64).unwrap()).map(|(s, e)| s..=e);

    let sum: u64 = input.flat_map(|range| range.filter(|id| is_invalid(*id))).sum();

    println!("{sum}");
}
