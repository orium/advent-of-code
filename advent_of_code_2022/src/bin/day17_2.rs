use itertools::Itertools;
use ndarray::{Array2, Axis};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};

const INPUT: &str = include_str!("../../inputs/17");

struct Shape {
    matrix: Array2<bool>,
    width: usize,
    height: usize,
}

impl Shape {
    fn new(matrix: Array2<bool>) -> Shape {
        let mut width = 0;
        let mut height = 0;

        for y in 0..matrix.len_of(Axis(1)) {
            for x in 0..matrix.len_of(Axis(0)) {
                if matrix[(x, y)] {
                    width = width.max(x + 1);
                    height = height.max(y + 1);
                }
            }
        }

        Shape { matrix, width, height }
    }

    fn a() -> Shape {
        let mut matrix = Array2::from_elem((4, 4), false);

        for x in 0..4 {
            matrix[(x, 0)] = true;
        }

        Shape::new(matrix)
    }

    fn b() -> Shape {
        let mut matrix = Array2::from_elem((4, 4), false);

        for y in 0..3 {
            matrix[(1, y)] = true;
        }

        for x in 0..3 {
            matrix[(x, 1)] = true;
        }

        Shape::new(matrix)
    }

    fn c() -> Shape {
        let mut matrix = Array2::from_elem((4, 4), false);

        for y in 0..3 {
            matrix[(2, y)] = true;
        }

        for x in 0..3 {
            matrix[(x, 2)] = true;
        }

        Shape::new(matrix)
    }

    fn d() -> Shape {
        let mut matrix = Array2::from_elem((4, 4), false);

        for y in 0..4 {
            matrix[(0, y)] = true;
        }

        Shape::new(matrix)
    }

    fn e() -> Shape {
        let mut matrix = Array2::from_elem((4, 4), false);

        for y in 0..2 {
            for x in 0..2 {
                matrix[(x, y)] = true;
            }
        }

        Shape::new(matrix)
    }

    fn used(&self, (x, y): (usize, usize)) -> bool {
        self.matrix[(x, y)]
    }
}

struct Chamber {
    matrix: Array2<bool>,
    highest: usize,
}

impl Chamber {
    fn new() -> Chamber {
        let mut matrix = Array2::from_elem((7 + 2, 10_000_000), false);
        let width = matrix.len_of(Axis(0));

        for y in 0..matrix.len_of(Axis(1)) {
            matrix[(0, y)] = true;
            matrix[(width - 1, y)] = true;
        }

        for x in 0..width {
            matrix[(x, 0)] = true;
        }

        Chamber { matrix, highest: 1 }
    }

    fn width(&self) -> usize {
        self.matrix.len_of(Axis(0))
    }

    fn highest(&self) -> usize {
        self.highest
    }

    fn collides(&self, shape: &Shape, (x, y): (usize, usize)) -> bool {
        for sy in 0..shape.height {
            for sx in 0..shape.width {
                if shape.used((sx, sy)) && self.matrix[(x + sx, y - sy)] {
                    return true;
                }
            }
        }

        false
    }

    fn add(&mut self, shape: &Shape, (x, y): (usize, usize)) {
        for sy in 0..shape.height {
            for sx in 0..shape.width {
                if shape.used((sx, sy)) {
                    self.matrix[(x + sx, y - sy)] = true;
                    self.highest = self.highest.max(y - sy + 1);
                }
            }
        }
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.width() {
            f.write_char('─')?;
        }

        f.write_char('\n')?;

        for y in (0..=self.highest()).rev() {
            for x in 0..self.width() {
                match self.matrix[(x, y)] {
                    true => f.write_char('█')?,
                    false => f.write_char(' ')?,
                }
            }
            f.write_char('\n')?;
        }

        for _ in 0..self.width() {
            f.write_char('─')?;
        }

        Ok(())
    }
}

fn char_to_dir(c: char) -> isize {
    match c {
        '<' => -1,
        '>' => 1,
        _ => unreachable!(),
    }
}

fn drop_shape(
    chamber: &mut Chamber,
    shape: &Shape,
    wind: &mut impl Iterator<Item = (usize, char)>,
) {
    let (mut x, mut y) = (3, 3 + shape.height + chamber.highest() - 1);

    while !chamber.collides(&shape, (x, y)) {
        let (_, ch) = wind.next().unwrap();
        let candidate_x = (x as isize + char_to_dir(ch)) as usize;

        if !chamber.collides(&shape, (candidate_x, y)) {
            x = candidate_x;
        }

        y -= 1;
    }

    y += 1;

    chamber.add(shape, (x, y));

    // println!("\n{}", chamber);
}

fn find_period(shapes: &[Shape], wind: &[char]) -> usize {
    let mut chamber = Chamber::new();
    let mut seen = HashMap::new();
    let mut wind_iter = wind.iter().copied().enumerate().cycle().peekable();
    let mut hit = 0;

    for (count, (si, shape)) in shapes.iter().enumerate().cycle().enumerate() {
        drop_shape(&mut chamber, shape, &mut wind_iter);
        let wi = wind_iter.peek().unwrap().0;

        if let Some(last_count) = seen.get(&(si, wi)) {
            let period = count - last_count;
            hit += 1;
            // Donºt know why we have to discard the first cycles ¯\_(ツ)_/¯
            if hit >= 10 {
                return period;
            }
        }

        seen.insert((si, wi), count);
    }

    unreachable!()
}

fn main() {
    let mut chamber = Chamber::new();
    let shapes = [Shape::a(), Shape::b(), Shape::c(), Shape::d(), Shape::e()];
    let wind = INPUT.chars().filter(|ch| !ch.is_whitespace()).collect_vec();
    let mut wind_iter = wind.iter().copied().enumerate().cycle().peekable();

    let mut before = 0;
    let mut delta: [usize; 2] = [0, 0];

    let period = find_period(&shapes, &wind);

    println!("period = {}", period);

    for count in 0..2 {
        for shape in shapes.iter().cycle().take(period) {
            drop_shape(&mut chamber, shape, &mut wind_iter);
        }

        let height = chamber.highest() - 1;
        delta[count] = height - before;

        println!("count = {} height = {}, delta = {}", count, height, delta[count]);
        before = height;
    }

    let drop_left = 1_000_000_000_000 - period;
    let remainder = drop_left % period;

    let before = chamber.highest() - 1;

    for shape in shapes.iter().cycle().take(remainder) {
        drop_shape(&mut chamber, shape, &mut wind_iter);
    }

    let now = chamber.highest() - 1;
    let last_drops = now - before;

    println!(
        "{} + {} * {} + {} = {}",
        delta[0],
        delta[1],
        drop_left / period,
        last_drops,
        delta[0] + delta[1] * (drop_left / period) + last_drops
    );
}
