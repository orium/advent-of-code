use scan_fmt::scan_fmt;

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

    fn map(&self, v: u64) -> Option<u64> {
        match (self.origin..self.origin + self.len).contains(&v) {
            true => Some(self.destination + (v - self.origin)),
            false => None,
        }
    }
}

#[derive(Debug)]
struct Layer {
    mappings: Vec<Mapping>,
}

impl Layer {
    fn empty() -> Layer {
        Layer { mappings: Vec::new() }
    }

    fn map(&self, v: u64) -> u64 {
        self.mappings.iter().find_map(|mapping| mapping.map(v)).unwrap_or(v)
    }
}

struct State {
    vs: Vec<u64>,
}

impl State {
    fn empty() -> State {
        State { vs: Vec::new() }
    }

    fn apply(mut self, layer: &Layer) -> State {
        for v in self.vs.iter_mut() {
            *v = layer.map(*v);
        }

        self
    }
}

fn load_input() -> (State, Vec<Layer>) {
    let mut initial_state = State::empty();
    let mut layers: Vec<Layer> = Vec::new();

    for line in INPUT.lines() {
        if line.starts_with("seeds:") {
            initial_state.vs.extend(line.split(' ').filter_map(|s| s.parse::<u64>().ok()));
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

    assert!(layers.iter().all(|layer| !layer.mappings.is_empty()));

    (initial_state, layers)
}

fn main() {
    let (initial_state, layers): (State, Vec<Layer>) = load_input();

    let final_state = layers.iter().fold(initial_state, State::apply);

    println!("{:?}", final_state.vs.iter().min());
}
