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

const MINS: u8 = 26;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    valves: u64,
    current_valve: [u8; 2],
    time: u8,
}

impl State {
    fn start(initial_valve: u8) -> State {
        State { valves: 0, current_valve: [initial_valve, initial_valve], time: 0 }
    }

    fn is_valve_open(&self, valve: u8) -> bool {
        ((self.valves >> valve) & 1) == 1
    }

    fn is_current_valve_open(&self, explorer: usize) -> bool {
        self.is_valve_open(self.current_valve[explorer])
    }

    fn open_valve(&self, valve: u8, flows: &Vec<u64>) -> (State, u64) {
        let mut s = self.clone();
        s.valves = s.valves | (1 << valve as u64);
        let relieved = flows[valve as usize] * MINS.saturating_sub(s.time) as u64;
        (s, relieved)
    }

    fn open_current_valve(&self, flows: &Vec<u64>, explorer: usize) -> (State, u64) {
        self.open_valve(self.current_valve[explorer], flows)
    }

    fn tick(&self) -> State {
        let mut s = self.clone();
        s.time += 1;
        s
    }

    fn move_to(&self, next_valve: u8, explorer: usize) -> State {
        let mut s = self.clone();
        s.current_valve[explorer] = next_valve;
        s
    }

    fn normalize(&mut self) {
        if self.current_valve[0] > self.current_valve[1] {
            self.current_valve.swap(0, 1);
        }
    }
}

fn successors_for_explorer<'a>(
    state: State,
    flows: &'a Vec<u64>,
    edges: &'a Vec<Vec<usize>>,
    explorer: usize,
) -> impl Iterator<Item = (State, u64)> + 'a {
    let is_valve_open = state.is_current_valve_open(explorer);

    let open_valve =
        std::iter::once(state.open_current_valve(flows, explorer)).filter(move |_| !is_valve_open);

    let ignore_valve = edges[state.current_valve[explorer] as usize]
        .iter()
        .map(move |&next_valve| state.move_to(next_valve as u8, explorer))
        .map(|state| (state, 0));

    ignore_valve.chain(open_valve).filter(|(s, _)| s.time <= MINS)
}

fn successors<'a>(
    state: &'a State,
    flows: &'a Vec<u64>,
    edges: &'a Vec<Vec<usize>>,
) -> impl Iterator<Item = (State, u64)> + 'a {
    let state = state.tick();

    successors_for_explorer(state, flows, edges, 0).flat_map(|(s, r1)| {
        successors_for_explorer(s, flows, edges, 1).map(move |(s, r2)| (s, r1 + r2))
    })
}

fn calc(
    state: &State,
    flows: &Vec<u64>,
    edges: &Vec<Vec<usize>>,
    memo: &mut HashMap<State, u64>,
) -> u64 {
    if let Some(&r) = memo.get(&state) {
        return r;
    }

    let mut best = 0;

    for (mut succ, relieved) in successors(state, flows, edges) {
        succ.normalize();
        best = best.max(relieved + calc(&succ, flows, edges, memo));
    }

    memo.insert(state.clone(), best);

    best
}

fn open_zero_flow_valves(state: State, flows: &Vec<u64>) -> State {
    let mut state = state;

    for (valve, &flow) in flows.iter().enumerate() {
        if flow == 0 {
            (state, _) = state.open_valve(valve as u8, flows);
        }
    }

    state
}

fn main() {
    let (flows, edges, start) = read_input();
    let initial_state = open_zero_flow_valves(State::start(start as u8), &flows);
    let mut memo: HashMap<State, u64> = HashMap::with_capacity(128_000_000);

    // This takes about 10mins to run on my current hardware.
    let best = calc(&initial_state, &flows, &edges, &mut memo);

    println!("{}", best);
}
