use ndarray::{Array2, Axis};
use std::fmt::{Display, Formatter, Write};
use std::iter::Peekable;

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
        let mut matrix = Array2::from_elem((7 + 2, 8192), false);
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

fn drop_shape(chamber: &mut Chamber, shape: &Shape, wind: &mut impl Iterator<Item = char>) {
    let (mut x, mut y) = (3, 3 + shape.height + chamber.highest() - 1);

    while !chamber.collides(&shape, (x, y)) {
        let ch = wind.next().unwrap();
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

fn main() {
    let mut chamber = Chamber::new();
    let shapes = [Shape::a(), Shape::b(), Shape::c(), Shape::d(), Shape::e()];
    let mut wind_iter = INPUT.chars().filter(|ch| !ch.is_whitespace()).cycle();

    for shape in shapes.iter().cycle().take(2022) {
        drop_shape(&mut chamber, shape, &mut wind_iter);
    }

    println!("{}", chamber.highest() - 1)
}
