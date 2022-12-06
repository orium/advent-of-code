const INPUT: &str = include_str!("../../inputs/04");

fn main() {
    let count =
        INPUT
            .lines()
            .map(|line| line.split_once(',').unwrap())
            .map(|(rl, rr)| (rl.split_once('-').unwrap(), rr.split_once('-').unwrap()))
            .map(|((a, b), (c, d))| (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap(), c.parse::<i64>().unwrap(), d.parse::<i64>().unwrap()))
            .map(|(a, b, c, d)| (a..=b, c..=d))
            .filter(|(rl, rr)|
                rl.contains(rr.start()) || rl.contains(rr.end()) || rr.contains(rl.start()) || rr.contains(rl.end()))
            .count();

    println!("{}", count);
}
