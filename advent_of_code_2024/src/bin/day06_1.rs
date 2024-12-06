use itertools::Itertools;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/06");

#[derive(Clone, Copy)]
enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    fn rotate(self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }

    fn as_deltas(self) -> (i32, i32) {
        match self {
            Orientation::Up => (0, 1),
            Orientation::Left => (-1, 0),
            Orientation::Down => (0, -1),
            Orientation::Right => (1, 0),
        }
    }
}

struct Guard {
    position: (i32, i32),
    orientation: Orientation,
}

impl Guard {
    fn ahead(&self) -> (i32, i32) {
        let (x, y) = self.position;
        let (dx, dy) = self.orientation.as_deltas();

        (x + dx, y + dy)
    }

    fn on(&self, position: (i32, i32)) -> Guard {
        Guard { position, orientation: self.orientation }
    }

    fn orient(&self, orientation: Orientation) -> Guard {
        Guard { position: self.position, orientation }
    }
}

struct Room {
    obstructions: HashSet<(i32, i32)>,
    height: u32,
    width: u32,
}

impl Room {
    fn within(&self, (x, y): (i32, i32)) -> bool {
        x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32
    }
}

fn load() -> (Room, Guard) {
    let height = INPUT.lines().count() as u32;
    let width = INPUT.lines().next().unwrap().len() as u32;

    let mut obstructions = HashSet::new();
    let mut guard = None;

    for (line_no, line) in INPUT.lines().enumerate() {
        let y = height - line_no as u32 - 1;
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '#' => {
                    obstructions.insert((x as i32, y as i32));
                }
                '^' => {
                    guard = Some(Guard {
                        position: (x as i32, y as i32),
                        orientation: Orientation::Up,
                    });
                }
                '.' => (),
                _ => unreachable!(),
            }
        }
    }

    let room = Room { obstructions, height, width };

    (room, guard.unwrap())
}

fn walk(room: &Room, guard: &Guard) -> Option<Guard> {
    Some(guard.ahead()).filter(|&pos| room.within(pos)).map(|pos| {
        match room.obstructions.contains(&pos) {
            true => guard.orient(guard.orientation.rotate()),
            false => guard.on(pos),
        }
    })
}

fn main() {
    let (room, guard) = load();

    let r = std::iter::successors(Some(guard), |guard| walk(&room, guard))
        .map(|guard| guard.position)
        .unique()
        .count();

    println!("{r}");
}
