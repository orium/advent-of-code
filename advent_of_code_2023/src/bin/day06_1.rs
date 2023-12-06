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
    let (times, distances): (Vec<u64>, Vec<u64>) = INPUT
        .lines()
        .map(|line| line.split(' ').filter_map(|s| s.parse().ok()).collect())
        .tuples()
        .next()
        .unwrap();

    let r: u64 = times.into_iter().zip(distances).map(|(t, d)| wins(t, d)).product();

    println!("{r}");
}
