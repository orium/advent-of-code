use nalgebra::RowVector3;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
struct Moon {
    position: RowVector3<isize>,
    velocity: RowVector3<isize>,
}

impl Moon {
    fn new_stationary(position: RowVector3<isize>) -> Moon {
        Moon { position, velocity: RowVector3::new(0, 0, 0) }
    }

    fn potential_energy(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) as usize
    }

    fn kinetic_energy(&self) -> usize {
        (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs()) as usize
    }

    fn total_energy(&self) -> usize {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Debug)]
struct Universe<'a> {
    moons: &'a mut [Moon],
}

impl<'a> Universe<'a> {
    fn new(moons: &'a mut [Moon]) -> Universe<'a> {
        Universe { moons }
    }

    fn total_energy(&self) -> usize {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }

    fn apply_gravity(&mut self) {
        fn delta(
            ref_moon: &Moon,
            other: &Moon,
            select: impl Fn(&RowVector3<isize>) -> isize,
        ) -> isize {
            let ref_v = select(&ref_moon.position);
            let other_v = select(&other.position);

            match ref_v.cmp(&other_v) {
                Ordering::Less => 1,
                Ordering::Equal => 0,
                Ordering::Greater => -1,
            }
        }

        for i in 0..self.moons.len() {
            for j in 0..self.moons.len() {
                self.moons[i].velocity.x += delta(&self.moons[i], &self.moons[j], |m| m.x);
                self.moons[i].velocity.y += delta(&self.moons[i], &self.moons[j], |m| m.y);
                self.moons[i].velocity.z += delta(&self.moons[i], &self.moons[j], |m| m.z);
            }
        }
    }

    fn move_step(&mut self) {
        for moon in self.moons.iter_mut() {
            moon.position += moon.velocity;
        }
    }

    fn advance(&mut self) {
        self.apply_gravity();
        self.move_step();
    }
}

fn main() -> io::Result<()> {
    let mut moons: [Moon; 4] = [
        Moon::new_stationary(RowVector3::new(-13, -13, -13)),
        Moon::new_stationary(RowVector3::new(5, -8, 3)),
        Moon::new_stationary(RowVector3::new(-6, -10, -3)),
        Moon::new_stationary(RowVector3::new(0, 5, -5)),
    ];

    let mut universe: Universe = Universe::new(&mut moons);

    for _ in 0..1000 {
        universe.advance();
    }

    println!("{}", universe.total_energy());

    Ok(())
}
