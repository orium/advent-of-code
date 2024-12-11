use itertools::Itertools;
use std::collections::HashMap;
use std::iter::successors;

fn apply(stone: u64) -> Box<dyn Iterator<Item = u64>> {
    let digits = || stone.ilog10() + 1;

    match stone {
        0 => Box::new([1].into_iter()),
        _ if digits() % 2 == 0 => {
            let mul = 10_u64.pow(digits() / 2);
            Box::new([stone / mul, stone % mul].into_iter())
        }
        _ => Box::new([2024 * stone].into_iter()),
    }
}

fn blink(stones: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    stones
        .iter()
        .flat_map(|(&stone, &count)| apply(stone).map(move |new_stone| (new_stone, count)))
        .into_grouping_map()
        .sum()
}

fn main() {
    let initial_stones = [510_613, 358, 84, 40_702, 4_373_582, 2, 0, 1_584].into_iter().counts();

    let r: usize = successors(Some(initial_stones), |stones| Some(blink(stones)))
        .nth(75)
        .unwrap()
        .into_values()
        .sum();

    println!("{r}");
}
