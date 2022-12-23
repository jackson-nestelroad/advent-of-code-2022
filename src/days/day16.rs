use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    str::FromStr,
};

use crate::common::{AocError, AocResult, IntoAocResult};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Valve {
    pub flow_rate: u64,
    pub tunnels: HashSet<String>,
}

#[derive(Debug)]
struct Volcano {
    valves: BTreeMap<String, Valve>,
}

impl<'a> FromStr for Volcano {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        lazy_static! {
            pub static ref PATTERN: Regex = Regex::new(
                r"^Valve ([A-Z]+) has flow rate=([0-9]+); tunnels? leads? to valves? ((?:[A-Z]+(?:, )?)+)$"
            ).unwrap();
        }

        Ok(Self {
            valves: s
                .lines()
                .map(|line| {
                    let captures = PATTERN
                        .captures(line)
                        .into_aoc_result_msg("input does not match expected pattern")?;
                    Ok((
                        captures[1].to_owned(),
                        Valve {
                            flow_rate: captures[2]
                                .parse()
                                .into_aoc_result_msg("invalid flow rate")?,
                            tunnels: captures[3]
                                .split(',')
                                .map(|s| s.trim().to_owned())
                                .collect(),
                        },
                    ))
                })
                .collect::<AocResult<BTreeMap<_, _>>>()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct PressureReleaseExplorationState {
    pub position: u8,
    pub valves_opened: u32,
    pub pressure_released: u32,
    pub time_remaining: u8,
}

impl PressureReleaseExplorationState {
    pub fn visited_and_opened(&self, valve: usize) -> bool {
        (self.valves_opened & (1 << valve)) != 0
    }

    pub fn move_to(&mut self, position: u8) {
        self.position = position
    }

    pub fn open(&mut self, valve: usize, pressure_released: u32) {
        self.valves_opened |= 1 << valve;
        self.pressure_released += pressure_released * (self.time_remaining as u32)
    }

    pub fn spend_time(&mut self, time: u8) {
        self.time_remaining = if time > self.time_remaining {
            0
        } else {
            self.time_remaining - time
        }
    }
}

#[derive(Debug)]
struct OptimizedVolcanoValveMap {
    pub starting_position_id: usize,
    pub valve_id_to_flow_rate: Vec<u64>,
    pub num_valves: usize,
    pub minimum_distances: Vec<u64>,
    pub valve_subset_to_relief: HashMap<u32, u32>,
}

impl OptimizedVolcanoValveMap {
    pub fn get_distance(&self, from: usize, to: usize) -> u64 {
        self.minimum_distances[from * self.num_valves + to]
    }

    pub fn get_distance_mut(&mut self, from: usize, to: usize) -> &mut u64 {
        &mut self.minimum_distances[from * self.num_valves + to]
    }

    fn initial_state(&self, minutes: u64) -> PressureReleaseExplorationState {
        let mut initial_state = PressureReleaseExplorationState {
            position: self.starting_position_id as u8,
            valves_opened: 0,
            pressure_released: 0,
            time_remaining: minutes as u8,
        };

        // If the flow rate at the start is 0, we don't need to explore states that may
        // or may not open it, so just mark it as open.
        if self.valve_id_to_flow_rate[self.starting_position_id] == 0 {
            initial_state.open(self.starting_position_id, 0);
        }

        initial_state
    }

    pub fn maximize_released_pressure(&mut self, minutes: u64) -> u64 {
        let start_state = self.initial_state(minutes);
        let mut queue = VecDeque::from([start_state.clone()]);
        let mut maximum_pressure_released = start_state.pressure_released;

        while let Some(state) = queue.pop_front() {
            if state.pressure_released > maximum_pressure_released {
                maximum_pressure_released = state.pressure_released;
            }

            // Used for part B.
            let max_pressure_relieved_at_subset = self
                .valve_subset_to_relief
                .entry(state.valves_opened)
                .or_insert(0);
            if state.pressure_released > *max_pressure_relieved_at_subset {
                *max_pressure_relieved_at_subset = state.pressure_released;
            }

            for (valve, flow_rate) in self.valve_id_to_flow_rate.iter().enumerate() {
                if !state.visited_and_opened(valve) {
                    let time = self.get_distance(state.position as usize, valve) as u8 + 1;
                    if state.time_remaining >= time {
                        let mut next_state = state.clone();
                        next_state.spend_time(time);
                        next_state.move_to(valve as u8);
                        next_state.open(valve, *flow_rate as u32);

                        queue.push_back(next_state);
                    }
                }
            }
        }

        maximum_pressure_released as u64
    }

    pub fn maximize_released_pressure_with_elephant(&mut self, minutes: u64) -> u64 {
        // First, visit all states as a single worker. This fills the
        // valve_subset_to_relief map.
        self.maximize_released_pressure(minutes);

        // If the valve at the starting position has no flow rate, then the above
        // algorithm only explores states where it is opened. Our disjoint sett will not
        // necessarily be disjoint in this case, since the starting valve will always be
        // open.
        let disjoint_state = if self.valve_id_to_flow_rate[self.starting_position_id] == 0 {
            1 << self.starting_position_id
        } else {
            0
        };

        // Our goal is for each subset of valves opened, take the disjoint set of valves
        // opened, which represents the elephant moving independently at the same time.
        self.valve_subset_to_relief
            .iter()
            .map(|(subset, pressure_released)| {
                // My initial idea was to just flip all of the bits of each subset. However,
                // this algorithm fails if there is no way for the two workers to open all
                // valves at once for different subsets. It will cause the real maximum, which
                // occurs when not all valves are opened by the two workers, to be missed.
                pressure_released
                    + self
                        .valve_subset_to_relief
                        .iter()
                        .filter(|(&other_subset, _)| subset & other_subset == disjoint_state)
                        .map(|(_, other_pressure_released)| other_pressure_released)
                        .max()
                        .unwrap_or(&0)
            })
            .max()
            .unwrap() as u64
    }
}

#[derive(Debug)]
struct VolcanoValveMap<'a> {
    pub volcano: &'a Volcano,
    pub valve_ids: BTreeMap<&'a str, usize>,
    pub num_valves: usize,
    pub minimum_distances: Vec<u64>,
}

impl<'a> VolcanoValveMap<'a> {
    fn initialize(volcano: &'a Volcano) -> Self {
        let valve_ids = volcano
            .valves
            .keys()
            .enumerate()
            .map(|(i, name)| (name.as_ref(), i))
            .collect::<BTreeMap<_, _>>();
        let num_valves = valve_ids.len();
        Self {
            volcano,
            valve_ids,
            num_valves,
            minimum_distances: vec![u64::MAX; num_valves * num_valves],
        }
    }

    pub fn get_distance(&self, from: usize, to: usize) -> u64 {
        self.minimum_distances[from * self.num_valves + to]
    }

    pub fn get_distance_mut(&mut self, from: usize, to: usize) -> &mut u64 {
        &mut self.minimum_distances[from * self.num_valves + to]
    }

    pub fn floyd_warshall_internal(&mut self, volcano: &'a Volcano) {
        for i in 0..self.num_valves {
            *self.get_distance_mut(i, i) = 0;
        }

        for (name, valve) in &volcano.valves {
            let from_id = self.valve_ids[name.as_str()];
            for connected in &valve.tunnels {
                let to_id = self.valve_ids[connected.as_str()];
                *self.get_distance_mut(from_id, to_id) = 1;
            }
        }

        for intermediate in 0..self.num_valves {
            for from in 0..self.num_valves {
                for to in 0..self.num_valves {
                    let distance_through_intermediate = self
                        .get_distance(from, intermediate)
                        .saturating_add(self.get_distance(intermediate, to));
                    if distance_through_intermediate < self.get_distance(from, to) {
                        *self.get_distance_mut(from, to) = distance_through_intermediate;
                    }
                }
            }
        }
    }

    pub fn floyd_warshall(volcano: &'a Volcano) -> Self {
        let mut map = Self::initialize(volcano);
        map.floyd_warshall_internal(volcano);
        map
    }

    pub fn optimize(self, starting_position: &str) -> OptimizedVolcanoValveMap {
        let included = self
            .volcano
            .valves
            .iter()
            .filter(|(name, valve)| name.as_str() == starting_position || valve.flow_rate != 0)
            .map(|(name, valve)| (name, self.valve_ids[name.as_str()], valve))
            .enumerate()
            .collect::<Vec<_>>();
        let num_included_valves = included.len();

        let mut optimized = OptimizedVolcanoValveMap {
            starting_position_id: usize::MAX,
            valve_id_to_flow_rate: vec![0; num_included_valves],
            num_valves: num_included_valves,
            minimum_distances: vec![0; num_included_valves * num_included_valves],
            valve_subset_to_relief: HashMap::new(),
        };

        for (new_id, (name, original_id, valve)) in &included {
            if name.as_str() == starting_position {
                optimized.starting_position_id = *new_id;
            }

            optimized.valve_id_to_flow_rate[*new_id] = valve.flow_rate;

            for (other_id, (_, other_original_id, _)) in &included {
                *optimized.get_distance_mut(*new_id, *other_id) =
                    self.get_distance(*original_id, *other_original_id)
            }
        }

        optimized
    }
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    const STARTING_POSITION: &str = "AA";
    const MINUTES: u64 = 30;
    let volcano = Volcano::from_str(input)?;
    let distance_map = VolcanoValveMap::floyd_warshall(&volcano);
    let mut optimized = distance_map.optimize(STARTING_POSITION);
    Ok(optimized.maximize_released_pressure(MINUTES))
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    const STARTING_POSITION: &str = "AA";
    const MINUTES: u64 = 26;
    let volcano = Volcano::from_str(input)?;
    let distance_map = VolcanoValveMap::floyd_warshall(&volcano);
    let mut optimized = distance_map.optimize(STARTING_POSITION);
    Ok(optimized.maximize_released_pressure_with_elephant(MINUTES))
}
