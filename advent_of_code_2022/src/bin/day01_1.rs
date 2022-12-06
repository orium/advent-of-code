const INPUT: &str = include_str!("../../inputs/01");

fn main() {
    let mut max: usize = 0;
    let mut current: usize = 0;

    for line in INPUT.lines() {
        match line.parse::<usize>() {
            Ok(num) => current += num,
            Err(_) => {
                max = max.max(current);
                current = 0;
            }
        }
    }

    assert_eq!(current, 0);

    println!("{}", max);
}
