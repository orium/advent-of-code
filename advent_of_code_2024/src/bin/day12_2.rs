use itertools::Itertools;
use ndarray::{Array2, Axis};

const INPUT: &str = include_str!("../../inputs/12");

fn load_matrix() -> Array2<char> {
    let height = INPUT.lines().count();
    let width = INPUT.lines().next().unwrap().chars().count();

    let mut matrix: Array2<char> = Array2::from_elem((width + 2, height + 2), '⋅');

    for (y, line) in INPUT.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            matrix[(x + 1, y + 1)] = ch;
        }
    }

    matrix
}

fn fill(map: &mut Array2<char>, new_region: char, (x, y): (usize, usize)) -> usize {
    let region: char = map[(x, y)];

    map[(x, y)] = new_region;

    [(1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(|(dx, dy)| ((x as isize + dx) as usize, (y as isize + dy) as usize))
        .map(|neighbour| match map[neighbour] == region {
            true => fill(map, new_region, neighbour),
            false => 0,
        })
        .sum::<usize>()
        + 1
}

#[allow(unused)]
fn print(map: &Array2<char>) {
    for y in 0..map.len_of(Axis(1)) {
        for x in 0..map.len_of(Axis(0)) {
            print!("{}", map[(x, y)]);
        }
        println!();
    }
}

fn scan(iter: impl Iterator<Item = impl Iterator<Item = char> + Clone> + Clone) -> usize {
    let count = |u, d| u != '█' && d == '█';

    iter.tuple_windows()
        .map(|(row_up, row_down)| {
            row_up
                .into_iter()
                .zip(row_down)
                .map(|(up, down)| count(up, down))
                .dedup()
                .filter(|v| *v)
                .count()
        })
        .sum()
}

fn sides(map: &Array2<char>) -> usize {
    let cols = map.columns().into_iter().map(|c| c.to_vec().into_iter());
    let rows = map.rows().into_iter().map(|c| c.to_vec().into_iter());

    scan(cols.clone()) + scan(rows.clone()) + scan(cols.rev()) + scan(rows.rev())
}

fn main() {
    let mut map = load_matrix();

    let r: usize = (0..map.len_of(Axis(0)))
        .cartesian_product(0..map.len_of(Axis(1)))
        .map(|pos| match map[pos] {
            '⋅' => (0, 0),
            _ => {
                let area = fill(&mut map, '█', pos);

                let sides = sides(&map);

                fill(&mut map, '⋅', pos);

                (sides, area)
            }
        })
        .map(|(sides, area)| sides * area)
        .sum();

    println!("{r}");
}
