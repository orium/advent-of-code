use std::ops::RangeInclusive;

struct Area {
    x_range: RangeInclusive<i64>,
    y_range: RangeInclusive<i64>,
}

impl Area {
    fn contains(&self, v: Vector) -> bool {
        self.x_range.contains(&v.x) && self.y_range.contains(&v.y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector {
    x: i64,
    y: i64,
}

fn position(initial_speed: Vector, t: i64) -> Vector {
    // The target area has positive x, so we don't have to bother with negative speed on the x axis.

    let tx = i64::min(t, initial_speed.x);

    Vector {
        x: initial_speed.x * tx - tx * (tx - 1) / 2,
        y: initial_speed.y * t - t * (t - 1) / 2,
    }
}

fn hits_target(initial_speed: Vector, target_area: &Area) -> bool {
    for t in 0.. {
        let pos = position(initial_speed, t);

        // Stop conditions only work for the the puzzle's conditions where the target area has
        // positive x and negative y.
        if pos.x > *target_area.x_range.end() || pos.y < *target_area.y_range.start() {
            break;
        }

        if target_area.contains(pos) {
            return true;
        }
    }

    false
}

fn main() {
    /*
    let target_area = Area {
        x_range: 20 ..= 30,
        y_range: -10 ..= -5
    };
    */
    let target_area = Area {
        x_range: 102..=157,
        y_range: -146..=-90
    };

    let mut count = 0;

    for y_speed in *target_area.y_range.start() .. {
        for x_speed in 1 ..= *target_area.x_range.end() {
            let initial_speed = Vector { x: x_speed, y: y_speed };
            if hits_target(initial_speed, &target_area) {
                count += 1;
                println!("{}", count);
            }
        }
    }
}

/*
 * initial speed = 7, 2
 * sx: 7, 6,  5,  4,  3,  2,  1,  0
 * x:  0, 7, 13, 18, 22, 25, 27, 28
 * y:  0, 2,  3,  3,  2,  0, -3, -7
 *
 * 0123456789012345678901234567890
 *
 * .............#....#............   3
 * .......#..............#........   2
 * ...............................   1
 * S........................#.....   0
 * ...............................  -1
 * ...............................  -2
 * ...........................#...  -3
 * ...............................  -4
 * ....................TTTTTTTTTTT  -5
 * ....................TTTTTTTTTTT  -6
 * ....................TTTTTTTT#TT  -7
 * ....................TTTTTTTTTTT  -8
 * ....................TTTTTTTTTTT  -9
 * ....................TTTTTTTTTTT -10
 */
