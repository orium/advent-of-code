use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/02");

fn is_safe_ascending(v: impl Iterator<Item = i64>) -> bool {
    v.tuple_windows().all(|(a, b)| 1 <= (b - a) && (b - a) <= 3)
}

fn is_safe(v: &[i64]) -> bool {
    is_safe_ascending(v.iter().copied()) || is_safe_ascending(v.iter().copied().rev())
}

fn main() {
    let r = INPUT
        .lines()
        .map(|line| line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .filter(|v| is_safe(v))
        .count();

    println!("{r}");
}
