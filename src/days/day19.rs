use std::{collections::VecDeque, str::FromStr};

use crate::common::{AocError, AocResult, IntoAocResult};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Material {
    #[default]
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

impl Material {
    pub const COUNT: usize = 4;

    pub fn index(&self) -> usize {
        *self as u8 as usize
    }
}

impl FromStr for Material {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        Ok(match s {
            "ore" => Self::Ore,
            "clay" => Self::Clay,
            "obsidian" => Self::Obsidian,
            "geode" => Self::Geode,
            _ => return Err(AocError::new(&format!("invalid material: {s}"))),
        })
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct RobotBlueprint {
    pub mines: Material,
    pub costs: [u64; Material::COUNT],
}

impl FromStr for RobotBlueprint {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        match s.split(' ').collect::<Vec<_>>().as_slice() {
            ["Each", mines, "robot", "costs", materials @ ..] => {
                let mines = Material::from_str(mines)?;
                let mut result = Self {
                    mines,
                    costs: [0; Material::COUNT],
                };
                let mut materials = materials;
                loop {
                    match materials {
                        [num, material, rest @ ..] => {
                            let num = num
                                .parse::<u64>()
                                .into_aoc_result_msg(&format!("invalid number: {num}"))?;
                            let material = Material::from_str(material)?;
                            result.costs[material.index()] = num;
                            match rest.first() {
                                Some(&"and") => materials = &rest[1..],
                                Some(word @ _) => {
                                    return Err(AocError::new(&format!(
                                        "invalid word after material: {word}"
                                    )))
                                }
                                None => break,
                            }
                        }
                        _ => return Err(AocError::new(&format!("invalid materials: {s}"))),
                    }
                }
                Ok(result)
            }
            _ => Err(AocError::new(&format!("invalid line: {s}"))),
        }
    }
}

#[derive(Debug)]
struct Blueprint {
    pub id: u64,
    pub robots: [RobotBlueprint; Material::COUNT],
}

impl FromStr for Blueprint {
    type Err = AocError;
    fn from_str(s: &str) -> AocResult<Self> {
        let (prefix, blueprint) = s
            .split_once(':')
            .into_aoc_result_msg(&format!("invalid blueprint: {s}"))?;
        let id = match prefix.split_once(' ') {
            Some(("Blueprint", num)) => num
                .parse()
                .into_aoc_result_msg(&format!("invalid blueprint id: {num}"))?,
            _ => {
                return Err(AocError::new(&format!(
                    "invalid blueprint prefix: {prefix}"
                )))
            }
        };

        let mut result = Self {
            id,
            robots: [RobotBlueprint::default(); Material::COUNT],
        };

        for robot in blueprint
            .trim()
            .split('.')
            .filter_map(|sentence| {
                let trimmed = sentence.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed)
                }
            })
            .map(|robot| RobotBlueprint::from_str(robot))
        {
            let robot = robot?;
            result.robots[robot.mines.index()] = robot;
        }

        Ok(result)
    }
}

impl Blueprint {
    pub fn quality_level(&self, material: Material, minutes: u64) -> u64 {
        self.maximize(material, minutes) * self.id
    }

    pub fn maximize(&self, material: Material, minutes: u64) -> u64 {
        BlueprintSimulation::new(self, material, minutes).maximize()
    }
}

#[derive(Debug, Clone)]
struct BlueprintSimulationState {
    pub minutes_passed: u64,
    pub inventory: [u64; Material::COUNT],
    pub robots: [u64; Material::COUNT],
}

impl BlueprintSimulationState {
    pub fn time_to_build_robot(&self, robot: &RobotBlueprint) -> u64 {
        robot
            .costs
            .iter()
            .enumerate()
            .map(|(i, cost)| {
                if self.inventory[i] >= *cost {
                    0
                } else {
                    let robots = self.robots[i];
                    if robots != 0 {
                        let needed = cost - self.inventory[i];
                        num::Integer::div_ceil(&needed, &robots)
                    } else {
                        u64::MAX
                    }
                }
            })
            .max()
            .unwrap_or(u64::MAX)
    }

    pub fn build_robot(&mut self, blueprint: &Blueprint, material: Material) {
        let robot_to_build = &blueprint.robots[material.index()];
        for (i, cost) in robot_to_build.costs.iter().enumerate() {
            self.inventory[i] -= cost;
        }
        self.robots[material.index()] += 1;
    }

    pub fn advance_time(&mut self, minutes: u64) {
        for (i, robots) in self.robots.iter().enumerate() {
            self.inventory[i] += robots * minutes;
        }
        self.minutes_passed += minutes;
    }
}

struct BlueprintSimulation<'a> {
    blueprint: &'a Blueprint,
    target: Material,
    minutes: u64,
    maximum_rates: [u64; Material::COUNT],
    best: u64,
}

impl<'a> BlueprintSimulation<'a> {
    pub fn new(blueprint: &'a Blueprint, target: Material, minutes: u64) -> Self {
        Self {
            blueprint,
            target,
            minutes,
            maximum_rates: [u64::MAX; Material::COUNT],
            best: 0,
        }
    }

    fn initialize_maximum_rates(&mut self) {
        for robot in &self.blueprint.robots {
            for (i, cost) in robot
                .costs
                .iter()
                .enumerate()
                .filter(|(_, &cost)| cost != 0)
            {
                let entry = &mut self.maximum_rates[i];
                if *entry == u64::MAX || cost > entry {
                    *entry = *cost;
                }
            }
        }
    }

    fn initial_state() -> BlueprintSimulationState {
        let mut state = BlueprintSimulationState {
            minutes_passed: 0,
            inventory: [0; Material::COUNT],
            robots: [0; Material::COUNT],
        };
        state.robots[Material::Ore.index()] = 1;
        state
    }

    fn handle_final_state(&mut self, state: BlueprintSimulationState) {
        let result = state.inventory[self.target.index()];
        if result > self.best {
            self.best = result;
        }
    }

    fn triangular_number(n: u64) -> u64 {
        n * (n + 1) / 2
    }

    pub fn run_simulation(&mut self) {
        // Explore multiple state paths.
        //
        // For each state, create one branch for each material, creating a new robot for
        // that material as soon as possible.
        //
        // There are several branch pruning rules detailed below.
        let mut states = VecDeque::from([Self::initial_state()]);

        while let Some(mut state) = states.pop_front() {
            let time_remaining = self.minutes - state.minutes_passed;

            // If we only have one second remaining, any robot we build is worthless.
            if time_remaining <= 1 {
                state.advance_time(time_remaining);
                self.handle_final_state(state);
                continue;
            }

            for robot in &self.blueprint.robots {
                // Building any robot that does not mine our target in the second-to-last minute
                // is worthless.
                if robot.mines != self.target && time_remaining <= 2 {
                    continue;
                }

                // Do not exceed the maximum rate we need for this material.
                if state.robots[robot.mines.index()] >= self.maximum_rates[robot.mines.index()] {
                    continue;
                }

                // There is absolutely no way we can beat our current best using this state.
                //
                // Current inventory of target material...
                // + how much material we will surely generate with our existing robots...
                // + how much material we will generate if we build one target-mining robot
                // every second (which is likely impossible, but represents the
                // ideal situation).
                //
                // If this sum is not greater than the current best, this state is worthless.
                if state.inventory[self.target.index()]
                    + state.robots[self.target.index()] * time_remaining
                    + Self::triangular_number(time_remaining)
                    <= self.best
                {
                    continue;
                }

                let mut next_state = state.clone();

                // Calculate the time it would take to build a new robot of this type.
                let delta_mins = next_state.time_to_build_robot(robot).saturating_add(1);
                if delta_mins < time_remaining {
                    // Enough time to build the robot and make use of it for at least one minute.
                    next_state.advance_time(delta_mins);
                    next_state.build_robot(self.blueprint, robot.mines);
                    states.push_back(next_state);
                } else {
                    // Cannot build a robot for this material, so this path is finished.
                    next_state.advance_time(time_remaining);
                    self.handle_final_state(next_state);
                }
            }
        }
    }

    pub fn maximize(&mut self) -> u64 {
        self.initialize_maximum_rates();
        self.run_simulation();
        self.best
    }
}

fn parse_blueprints(input: &str) -> AocResult<Vec<Blueprint>> {
    input
        .lines()
        .map(|line| Blueprint::from_str(line))
        .collect()
}

pub fn solve_a(input: &str) -> AocResult<u64> {
    let blueprints = parse_blueprints(input)?;
    Ok(blueprints
        .into_iter()
        .map(|blueprint| blueprint.quality_level(Material::Geode, 24))
        .sum())
}

pub fn solve_b(input: &str) -> AocResult<u64> {
    let blueprints = parse_blueprints(input)?;
    Ok(blueprints
        .into_iter()
        .take(3)
        .map(|blueprint| blueprint.maximize(Material::Geode, 32))
        .product())
}
