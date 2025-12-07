use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/06");

fn main() {
    let input = INPUT.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let width = input.iter().map(Vec::len).max().unwrap();
    let height = input.len() - 1;
    let nums = (0..width)
        .map(|c| {
            (0..height)
                .map(|r| input.get(r).and_then(|row| row.get(c)).copied().unwrap_or('0'))
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .ok()
        })
        .collect_vec();
    let ops = INPUT
        .lines()
        .last()
        .unwrap()
        .chars()
        .filter_map(|c| match c {
            '+' => Some(Box::<fn(u64, u64) -> u64>::new(|a, b| a + b)),
            '*' => Some(Box::<fn(u64, u64) -> u64>::new(|a, b| a * b)),
            ' ' => None,
            _ => unreachable!(),
        })
        .collect_vec();

    let r: u64 = nums
        .split(Option::is_none)
        .map(|ns| ns.iter().map(|n| n.unwrap()).collect_vec())
        .enumerate()
        .map(|(col, nums)| nums.into_iter().reduce(*ops[col]).unwrap_or_default())
        .sum();

    println!("{r}");
}
