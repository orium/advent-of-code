use std::collections::BinaryHeap;

const INPUT: &str = include_str!("../../inputs/01");

fn main() {
    let mut cals: BinaryHeap<usize> = BinaryHeap::new();
    let mut current: usize = 0;

    for line in INPUT.lines() {
        match line.parse::<usize>() {
            Ok(num) => current += num,
            Err(_) => {
                cals.push(current);
                current = 0;
            }
        }
    }

    assert_eq!(current, 0);

    println!("{}", (0..3).filter_map(|_| cals.pop()).sum::<usize>());
}
