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

        println!("current {}", current);
        println!("{line}");

        let inc = match direction {
            -1 if current == 0 => (100 - current + amount) / 100 - 1,
            -1 => (100 - current + amount) / 100,
            1 => (current + amount) / 100,
            _ => unreachable!(),
        };

        zero_count += inc;

        current = (current + amount * direction).rem_euclid(100);
    }

    println!("{zero_count}");
}
