use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/13");

fn solve(ax: u64, ay: u64, bx: u64, by: u64, x: u64, y: u64) -> Option<u64> {
    (0..=100)
        .cartesian_product(0..=100)
        .filter(|(a, b)| a * ax + b * bx == x && a * ay + b * by == y)
        .map(|(a, b)| 3 * a + b)
        .min()
}

fn main() {
    let r: u64 = INPUT
        .split("\n\n")
        .filter_map(|case| {
            let case = case.lines().collect_vec();
            let (ax, ay) = scan_fmt!(case[0], "Button A: X+{}, Y+{}", u64, u64).unwrap();
            let (bx, by) = scan_fmt!(case[1], "Button B: X+{}, Y+{}", u64, u64).unwrap();
            let (x, y) = scan_fmt!(case[2], "Prize: X={}, Y={}", u64, u64).unwrap();

            solve(ax, ay, bx, by, x, y)
        })
        .sum();

    println!("{r}");
}
