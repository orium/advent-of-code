use scan_fmt::scan_fmt;
use std::collections::HashMap;

const INPUT: &str = include_str!("../../inputs/16");

fn read_input() -> (Vec<u64>, Vec<Vec<usize>>, usize) {
    let mut valves_indexes: HashMap<String, usize> = HashMap::new();
    let mut valve_count = 0;

    let mut index_of_valve = |valve: &str| -> usize {
        *valves_indexes.entry(valve.to_owned()).or_insert_with(|| {
            let v = valve_count;
            valve_count += 1;
            v
        })
    };

    let mut flows: Vec<u64> = std::vec::from_elem(0, 64);
    let mut edges: Vec<Vec<usize>> = std::vec::from_elem(Vec::new(), 64);
    let mut start = None;

    for line in INPUT.lines() {
        let (valve_str, flow, dests) = scan_fmt!(
            line,
            "Valve {} has flow rate={}; {*} {*} to {*} {/.*/}",
            String,
            u64,
            String
        )
        .unwrap();

        let valve = index_of_valve(&valve_str);
        let dests = dests.split(", ").map(&mut index_of_valve);

        flows[valve] = flow;
        edges[valve].extend(dests);

        if valve_str == "AA" {
            start = Some(valve);
        }
    }

    flows.truncate(valve_count);
    edges.truncate(valve_count);

    (flows, edges, start.unwrap())
}

const MINS: u8 = 30;

type StateId = (u64, usize, u8);

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    valves: u64,
    current_valve: usize,
    time: u8,
    relieved: u64,
}

impl State {
    fn start(initial_valve: usize) -> State {
        State { valves: 0, current_valve: initial_valve, time: 0, relieved: 0 }
    }

    fn is_valve_open(&self, valve: usize) -> bool {
        ((self.valves >> valve) & 1) == 1
    }

    fn is_current_valve_open(&self) -> bool {
        self.is_valve_open(self.current_valve)
    }

    fn open_valve(&self, valve: usize, flows: &Vec<u64>) -> State {
        let mut s = self.clone();
        s.valves = s.valves | (1 << valve as u64);
        s.relieved += flows[valve] * MINS.saturating_sub(s.time) as u64;
        s
    }

    fn open_current_valve(&self, flows: &Vec<u64>) -> State {
        self.open_valve(self.current_valve, flows)
    }

    fn tick(&self) -> State {
        let mut s = self.clone();
        s.time += 1;
        s
    }

    fn move_to(&self, next_valve: usize) -> State {
        let mut s = self.clone();
        s.current_valve = next_valve;
        s
    }

    fn identity(&self) -> StateId {
        (self.valves, self.current_valve, self.time)
    }
}

fn successors<'a>(
    state: &'a State,
    flows: &'a Vec<u64>,
    edges: &'a Vec<Vec<usize>>,
) -> impl Iterator<Item = State> + 'a {
    let state = state.tick();
    let is_valve_open = state.is_current_valve_open();

    let open_valve =
        std::iter::once(state.open_current_valve(flows)).filter(move |_| !is_valve_open);

    let ignore_valve =
        edges[state.current_valve].iter().map(move |&next_valve| state.move_to(next_valve));

    ignore_valve.chain(open_valve).filter(|s| s.time <= MINS)
}

fn dfs(state: &State, flows: &Vec<u64>, edges: &Vec<Vec<usize>>, best: &mut HashMap<StateId, u64>) {
    if best.get(&state.identity()).map(|&b| state.relieved <= b).unwrap_or(false) {
        return;
    }

    best.insert(state.identity(), state.relieved);

    for s in successors(state, flows, edges) {
        dfs(&s, flows, edges, best);
    }
}

fn open_zero_flow_valves(state: State, flows: &Vec<u64>) -> State {
    let mut state = state;

    for (valve, &flow) in flows.iter().enumerate() {
        if flow == 0 {
            state = state.open_valve(valve, flows);
        }
    }

    state
}

fn main() {
    let (flows, edges, start) = read_input();
    let initial_state = open_zero_flow_valves(State::start(start), &flows);
    let mut best: HashMap<StateId, u64> = HashMap::new();

    dfs(&initial_state, &flows, &edges, &mut best);

    let max = best.values().max();

    println!("{:?}", max);
}
