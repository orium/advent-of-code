use itertools::Itertools;
use scan_fmt::scan_fmt;

const INPUT: &str = include_str!("../../inputs/13");

fn solve(ax: u64, ay: u64, bx: u64, by: u64, x: u64, y: u64) -> Option<u64> {
    // We want to find a and b such that
    //
    //   a⋅ax + b⋅bx = x
    //   a⋅ay + b⋅by = y
    //
    // Lets derive b from a:
    //
    //   b = (x - a⋅ax) / bx
    //   b = (y - a⋅ay) / by

    println!("search space: {}", x / ax + 1);

    (0..=(x / ax + 1))
        .filter(|a| x >= a * ax && y >= a * ay)
        .filter(|a| (x - a * ax) % bx == 0 && (y - a * ay) % by == 0)
        .filter(|a| (x - a * ax) / bx == (y - a * ay) / by)
        .map(|a| {
            let b = (x - a * ax) / bx;

            3 * a + b
        })
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
            let (x, y) = (x + 10_000_000_000_000, y + 10_000_000_000_000);

            solve(ax, ay, bx, by, x, y)
        })
        .sum();

    println!("{r}");
}
