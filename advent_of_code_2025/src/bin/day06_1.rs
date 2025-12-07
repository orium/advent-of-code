use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/06");

fn main() {
    let nums = INPUT
        .lines()
        .map(|l| l.split(' ').filter_map(|v| v.parse::<u64>().ok()).collect_vec())
        .filter(|nums| !nums.is_empty())
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

    let r: u64 = (0..nums[0].len())
        .map(|col| nums.iter().map(|line| line[col]).reduce(*ops[col]).unwrap_or_default())
        .sum();

    println!("{r}");
}
