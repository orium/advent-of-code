use itertools::Itertools;
use num::ToPrimitive;
use scan_fmt::scan_fmt;
use std::ops::Range;

const INPUT: &str = include_str!("../../inputs/05");

#[derive(Debug)]
struct Mapping {
    origin: u64,
    destination: u64,
    len: u64,
}

impl Mapping {
    fn new(origin: u64, destination: u64, len: u64) -> Mapping {
        Mapping { origin, destination, len }
    }

    fn identity(range: Range<u64>) -> Mapping {
        Mapping::new(range.start, range.start, range.end - range.start)
    }

    fn input_range(&self) -> Range<u64> {
        self.origin..(self.origin + self.len)
    }

    fn shift(&self) -> i64 {
        self.destination.to_i64().unwrap() - self.origin.to_i64().unwrap()
    }
}

#[derive(Debug)]
struct Layer {
    mappings: Vec<Mapping>,
}

fn range_shift(range: &Range<u64>, shift: i64) -> Range<u64> {
    range.start.checked_add_signed(shift).unwrap()..range.end.checked_add_signed(shift).unwrap()
}

impl Layer {
    fn empty() -> Layer {
        Layer { mappings: Vec::new() }
    }

    fn fill_in_gaps(self) -> Layer {
        let mut new_layer = Layer::empty();
        let mut last_end = 0;

        for mapping in self.mappings.into_iter().sorted_by_key(|mapping| mapping.origin) {
            let filler_range = last_end..mapping.input_range().start;

            if !filler_range.is_empty() {
                new_layer.mappings.push(Mapping::identity(filler_range));
            }

            last_end = mapping.input_range().end;

            new_layer.mappings.push(mapping);
        }

        new_layer.mappings.push(Mapping::identity(last_end..u64::MAX));

        new_layer
    }

    fn apply(&self, input_state: State) -> State {
        let mut output_state = State::empty();

        for range in input_state.ranges {
            for mapping in &self.mappings {
                let intersection = range_intersection(&range, &mapping.input_range());

                if !intersection.is_empty() {
                    let output_range = range_shift(&intersection, mapping.shift());

                    output_state.ranges.push(output_range);
                }
            }
        }

        output_state
    }
}

fn range_intersection(a: &Range<u64>, b: &Range<u64>) -> Range<u64> {
    a.start.max(b.start)..a.end.min(b.end)
}

#[derive(Debug)]
struct State {
    ranges: Vec<Range<u64>>,
}

impl State {
    fn empty() -> State {
        State { ranges: Vec::new() }
    }

    fn min(&self) -> Option<u64> {
        self.ranges.iter().map(|s| s.start).min()
    }
}

fn load_input() -> (State, Vec<Layer>) {
    let mut initial_state: Option<State> = None;
    let mut layers: Vec<Layer> = Vec::new();

    for line in INPUT.lines() {
        if line.starts_with("seeds:") {
            initial_state = Some(State {
                ranges: line
                    .split(' ')
                    .filter_map(|s| s.parse::<u64>().ok())
                    .tuples()
                    .map(|(v, l)| v..(v + l))
                    .collect(),
            });
        }

        match line.trim().is_empty() {
            true => layers.push(Layer::empty()),
            false => {
                if let Ok((dest, orig, len)) = scan_fmt!(line, "{d} {d} {d}", u64, u64, u64) {
                    let mapping = Mapping::new(orig, dest, len);

                    layers.last_mut().unwrap().mappings.push(mapping);
                }
            }
        }
    }

    assert!(!layers.last().unwrap().mappings.is_empty());

    (initial_state.unwrap(), layers)
}

fn main() {
    let (initial_state, layers): (State, Vec<Layer>) = load_input();

    let final_state = layers
        .into_iter()
        .map(Layer::fill_in_gaps)
        .fold(initial_state, |state, layer| layer.apply(state));

    println!("{}", final_state.min().unwrap());
}
