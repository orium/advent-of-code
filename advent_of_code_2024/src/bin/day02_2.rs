use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/02");

fn is_safe_ascending(v: impl Iterator<Item = i64>) -> bool {
    v.tuple_windows().all(|(a, b)| 1 <= (b - a) && (b - a) <= 3)
}

fn jump_nth(
    iter: impl DoubleEndedIterator<Item = i64> + ExactSizeIterator,
    skip: usize,
) -> impl DoubleEndedIterator<Item = i64> {
    iter.enumerate().filter(move |(i, _)| *i != skip).map(|(_, v)| v)
}

fn is_safe(seq: &[i64]) -> bool {
    (0..=seq.len()).any(|skip| {
        let seq_skipped = || jump_nth(seq.iter().copied(), skip);

        is_safe_ascending(seq_skipped()) || is_safe_ascending(seq_skipped().rev())
    })
}

fn main() {
    let r = INPUT
        .lines()
        .map(|line| line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .filter(|v| is_safe(v))
        .count();

    println!("{r}");
}
