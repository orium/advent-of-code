use itertools::Itertools;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/08");

fn load() -> Map {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let height = INPUT.lines().count() as i32;
    let width = INPUT.lines().next().unwrap().len() as i32;

    for (x, line) in INPUT.lines().enumerate() {
        for (y, ch) in line.chars().enumerate() {
            if ch.is_alphanumeric() {
                antennas.entry(ch).or_default().push((x as i32, y as i32));
            }
        }
    }

    Map { antennas, height, width }
}

struct Map {
    antennas: HashMap<char, Vec<(i32, i32)>>,
    height: i32,
    width: i32,
}

impl Map {
    fn within(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }
}

fn left_antinodes(
    map: &Map,
    (ax, ay): (i32, i32),
    (bx, by): (i32, i32),
) -> impl Iterator<Item = (i32, i32)> + '_ {
    let (dx, dy) = (bx - ax, by - ay);

    (0..)
        .map(move |multiple| (ax - multiple * dx, ay - multiple * dy))
        .take_while(|&pos| map.within(pos))
}

fn main() {
    let map = load();

    let r = map
        .antennas
        .values()
        .flat_map(|same_freq_antennas| {
            same_freq_antennas
                .iter()
                .cartesian_product(same_freq_antennas)
                .filter(|(a, b)| a != b)
                .flat_map(|(&a, &b)| left_antinodes(&map, a, b))
        })
        .unique()
        .count();

    println!("{r}");
}
