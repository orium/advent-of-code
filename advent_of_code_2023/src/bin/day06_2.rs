use itertools::Itertools;

const INPUT: &str = include_str!("../../inputs/06");

fn wins(best_time: u64, best_distance: u64) -> u64 {
    (0..=best_time)
        .map(|charge_time| {
            let run_time = best_time - charge_time;
            let speed = charge_time;

            run_time * speed
        })
        .filter(|distance| *distance > best_distance)
        .count() as u64
}

fn main() {
    let (time, distance): (u64, u64) = INPUT
        .lines()
        .map(|line| line.chars().filter(char::is_ascii_digit).collect::<String>())
        .filter_map(|line| line.parse().ok())
        .tuples()
        .next()
        .unwrap();

    println!("time: {time}");
    println!("distance: {distance}");

    let r: u64 = wins(time, distance);

    println!("{r}");
}
