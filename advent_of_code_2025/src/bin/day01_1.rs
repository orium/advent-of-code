const INPUT: &str = include_str!("../../inputs/01");

fn main() {
    let mut zero_count = 0;
    let mut current = 50;

    for line in INPUT.lines() {
        let direction = match line.chars().next().unwrap() {
            'L' => -1,
            'R' => 1,
            _ => unreachable!(),
        };
        let amount: i32 = line[1..].parse().unwrap();

        current = (current + amount * direction).rem_euclid(100);

        if current == 0 {
            zero_count += 1;
        }
    }

    println!("{zero_count}");
}
