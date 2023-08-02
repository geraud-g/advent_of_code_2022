use advent_of_code::utils::inputs::get_file;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::max;

lazy_static! {
    static ref RE_PARSE_LINE: Regex = Regex::new(r"[[a-zA-Z] ]+(\d+)[[a-zA-Z] :]+(\d+)[[a-zA-Z] .]+(\d+)[[a-zA-Z] .]+(\d+)[[a-zA-Z] ]+(\d+)[[a-zA-Z] .]+(\d+)[[a-zA-Z] ]+(\d+)").unwrap();
}

pub fn day_19() {
    let blueprints = get_input("./src/day_19/input.txt");
    let solution_1 = part_one(&blueprints);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(&blueprints);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn part_one(blueprints: &[Blueprint]) -> i16 {
    blueprints
        .par_iter()
        .map(|b| get_max_geodes(b, State::new(), 24) * b.id)
        .sum()
}

fn part_two(blueprints: &[Blueprint]) -> i16 {
    blueprints
        .par_iter()
        .take(3)
        .map(|b| get_max_geodes(b, State::new(), 32))
        .product()
}

#[derive(Debug, Clone)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

const RESOURCES_TYPES: [ResourceType; 4] = [
    ResourceType::Ore,
    ResourceType::Clay,
    ResourceType::Obsidian,
    ResourceType::Geode,
];

#[derive(Debug, Clone)]
struct Robot {
    ore_cost: i16,
    clay_cost: i16,
    obsidian_cost: i16,
}

impl Robot {
    fn new(ore_cost: i16, clay_cost: i16, obsidian_cost: i16) -> Self {
        Robot {
            ore_cost,
            clay_cost,
            obsidian_cost,
        }
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: i16,
    robot_ore: Robot,
    robot_clay: Robot,
    robot_obsidian: Robot,
    robot_geode: Robot,
    max_robot_ore: i16,
    max_robot_clay: i16,
    max_robot_obsidian: i16,
}

impl Blueprint {
    fn new(
        id: i16,
        robot_ore: Robot,
        robot_clay: Robot,
        robot_obsidian: Robot,
        robot_geode: Robot,
    ) -> Self {
        let max_robot_ore = *[
            robot_ore.ore_cost,
            robot_clay.ore_cost,
            robot_obsidian.ore_cost,
            robot_geode.ore_cost,
        ]
        .iter()
        .max()
        .unwrap();
        let max_robot_clay = robot_obsidian.clay_cost;
        let max_robot_obsidian = robot_geode.obsidian_cost;

        Self {
            id,
            robot_ore,
            robot_clay,
            robot_obsidian,
            robot_geode,
            max_robot_ore,
            max_robot_clay,
            max_robot_obsidian,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    robots_ore: i16,
    robots_clay: i16,
    robots_obsidian: i16,
    robots_geode: i16,
    ore: i16,
    clay: i16,
    obsidian: i16,
    geode: i16,
    turn: i16,
}

impl State {
    fn new() -> Self {
        Self {
            robots_ore: 1,
            robots_clay: 0,
            robots_obsidian: 0,
            robots_geode: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            turn: 0,
        }
    }

    /// If a robot can be built at some point in the future, return the state matching this build.
    fn get_state_with_built_robot(
        &self,
        blueprint: &Blueprint,
        resource_type: &ResourceType,
        max_turn: i16,
    ) -> Option<Self> {
        if let Some(turns_to_wait) = self.turns_to_get_robot(blueprint, resource_type) {
            let mut new_state = self.get_state_after_n_turns(turns_to_wait + 1);
            let is_geode = matches!(resource_type, ResourceType::Geode);
            if (new_state.turn >= max_turn - 1 && !is_geode) || new_state.turn >= max_turn {
                return None;
            }
            new_state.build_robot(blueprint, resource_type);
            Some(new_state)
        } else {
            None
        }
    }

    fn turns_to_get_robot(&self, blueprint: &Blueprint, robot_type: &ResourceType) -> Option<i16> {
        match robot_type {
            ResourceType::Ore => {
                if self.robots_ore >= blueprint.max_robot_ore {
                    return None;
                }
                // We always have at least one robot ore, no need to check if > 0
                let required_ore = blueprint.robot_ore.ore_cost - self.ore;
                let turns_to_robot = divide_and_round_up(required_ore, self.robots_ore);
                Some(turns_to_robot)
            }
            ResourceType::Clay => {
                if self.robots_clay >= blueprint.max_robot_clay {
                    return None;
                }
                // We always have at least one robot ore, no need to check if > 0
                let required_ore = blueprint.robot_clay.ore_cost - self.ore;
                let turns_to_robot = divide_and_round_up(required_ore, self.robots_ore);
                Some(turns_to_robot)
            }
            ResourceType::Obsidian => {
                if self.robots_clay == 0 || self.robots_obsidian >= blueprint.max_robot_obsidian {
                    return None;
                }
                let required_ore = blueprint.robot_obsidian.ore_cost - self.ore;
                let turns_to_robot_ore = divide_and_round_up(required_ore, self.robots_ore);

                let required_clay = blueprint.robot_obsidian.clay_cost - self.clay;
                let turns_to_robot_clay = divide_and_round_up(required_clay, self.robots_clay);

                let min_required_turn = max(turns_to_robot_ore, turns_to_robot_clay);
                Some(min_required_turn)
            }
            ResourceType::Geode => {
                if self.robots_obsidian == 0 {
                    None
                } else {
                    let required_ore = blueprint.robot_geode.ore_cost - self.ore;
                    let turns_to_robot_ore = divide_and_round_up(required_ore, self.robots_ore);

                    let required_obsidian = blueprint.robot_geode.obsidian_cost - self.obsidian;
                    let turns_to_robot_obsidian =
                        divide_and_round_up(required_obsidian, self.robots_obsidian);
                    let min_required_turn = max(turns_to_robot_ore, turns_to_robot_obsidian);
                    Some(min_required_turn)
                }
            }
        }
    }

    fn build_robot(&mut self, blueprint: &Blueprint, robot_type: &ResourceType) {
        match robot_type {
            ResourceType::Ore => {
                self.ore -= blueprint.robot_ore.ore_cost;
                self.robots_ore += 1;
            }
            ResourceType::Clay => {
                self.ore -= blueprint.robot_clay.ore_cost;
                self.robots_clay += 1;
            }
            ResourceType::Obsidian => {
                self.ore -= blueprint.robot_obsidian.ore_cost;
                self.clay -= blueprint.robot_obsidian.clay_cost;
                self.robots_obsidian += 1;
            }
            ResourceType::Geode => {
                self.ore -= blueprint.robot_geode.ore_cost;
                self.obsidian -= blueprint.robot_geode.obsidian_cost;
                self.robots_geode += 1;
            }
        }
    }

    fn get_state_after_n_turns(&self, turns_to_wait: i16) -> Self {
        Self {
            ore: self.ore + self.robots_ore * turns_to_wait,
            clay: self.clay + self.robots_clay * turns_to_wait,
            obsidian: self.obsidian + self.robots_obsidian * turns_to_wait,
            geode: self.geode + self.robots_geode * turns_to_wait,
            turn: self.turn + turns_to_wait,
            ..*self
        }
    }
}

fn divide_and_round_up(dividend: i16, divisor: i16) -> i16 {
    max((dividend + divisor - 1) / divisor, 0)
}

fn get_input(file: &str) -> Vec<Blueprint> {
    get_file(file).lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Blueprint {
    let values: Vec<_> = RE_PARSE_LINE
        .captures(line)
        .unwrap_or_else(|| panic!("Cannot parse line {}", line))
        .iter()
        .skip(1)
        .filter_map(|value| value.as_ref().and_then(|m| m.as_str().parse::<i16>().ok()))
        .collect();

    Blueprint::new(
        values[0],
        Robot::new(values[1], 0, 0),
        Robot::new(values[2], 0, 0),
        Robot::new(values[3], values[4], 0),
        Robot::new(values[5], 0, values[6]),
    )
}

fn get_max_geodes(blueprint: &Blueprint, state: State, max_turn: i16) -> i16 {
    let max_geodes = RESOURCES_TYPES
        .iter()
        .filter_map(|r| state.get_state_with_built_robot(blueprint, r, max_turn))
        .map(|new_state| get_max_geodes(blueprint, new_state, max_turn))
        .max()
        .unwrap_or(0);

    let new_state = state.get_state_after_n_turns(max_turn - state.turn);
    max(max_geodes, new_state.geode)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elapse_n_turns() {
        let mut resource = State::new();
        assert_eq!(resource.turn, 0);
        assert_eq!(resource.ore, 0);
        assert_eq!(resource.clay, 0);
        assert_eq!(resource.obsidian, 0);
        assert_eq!(resource.geode, 0);

        resource = resource.get_state_after_n_turns(1);
        assert_eq!(resource.turn, 1);
        assert_eq!(resource.ore, 1);
        assert_eq!(resource.clay, 0);
        assert_eq!(resource.obsidian, 0);
        assert_eq!(resource.geode, 0);

        resource.robots_ore += 1;
        resource.robots_clay += 3;
        resource.robots_obsidian += 4;
        resource.robots_geode += 5;

        resource = resource.get_state_after_n_turns(3);
        assert_eq!(resource.robots_ore, 2);
        assert_eq!(resource.robots_clay, 3);
        assert_eq!(resource.robots_obsidian, 4);
        assert_eq!(resource.robots_geode, 5);

        assert_eq!(resource.turn, 4);
        assert_eq!(resource.ore, 7);
        assert_eq!(resource.clay, 9);
        assert_eq!(resource.obsidian, 12);
        assert_eq!(resource.geode, 15);
    }

    #[test]
    fn test_can_afford_in_future_with_example() {
        let blueprint = get_input("./src/day_19/input_example.txt")[0].clone();
        let state = State::new();

        assert!(state
            .get_state_with_built_robot(&blueprint, &ResourceType::Ore, 24)
            .is_some());
        assert!(state
            .get_state_with_built_robot(&blueprint, &ResourceType::Clay, 24)
            .is_some());
        assert!(state
            .get_state_with_built_robot(&blueprint, &ResourceType::Obsidian, 24)
            .is_none());
        assert!(state
            .get_state_with_built_robot(&blueprint, &ResourceType::Geode, 24)
            .is_none());

        let turn_3 = state
            .get_state_with_built_robot(&blueprint, &ResourceType::Clay, 24)
            .unwrap();
        assert_eq!(turn_3.turn, 3);
        assert_eq!(turn_3.robots_ore, 1);
        assert_eq!(turn_3.robots_clay, 1);
        assert_eq!(turn_3.robots_obsidian, 0);
        assert_eq!(turn_3.robots_geode, 0);
        assert_eq!(turn_3.ore, 1);
        assert_eq!(turn_3.clay, 0);
        assert_eq!(turn_3.obsidian, 0);
        assert_eq!(turn_3.geode, 0);

        let turn_5 = turn_3
            .get_state_with_built_robot(&blueprint, &ResourceType::Clay, 24)
            .unwrap();
        assert_eq!(turn_5.turn, 5);
        assert_eq!(turn_5.robots_ore, 1);
        assert_eq!(turn_5.robots_clay, 2);
        assert_eq!(turn_5.robots_obsidian, 0);
        assert_eq!(turn_5.robots_geode, 0);
        assert_eq!(turn_5.ore, 1);
        assert_eq!(turn_5.clay, 2);
        assert_eq!(turn_5.obsidian, 0);
        assert_eq!(turn_5.geode, 0);

        let turn_7 = turn_5
            .get_state_with_built_robot(&blueprint, &ResourceType::Clay, 24)
            .unwrap();
        assert_eq!(turn_7.turn, 7);
        assert_eq!(turn_7.robots_ore, 1);
        assert_eq!(turn_7.robots_clay, 3);
        assert_eq!(turn_7.robots_obsidian, 0);
        assert_eq!(turn_7.robots_geode, 0);
        assert_eq!(turn_7.ore, 1);
        assert_eq!(turn_7.clay, 6);
        assert_eq!(turn_7.obsidian, 0);
        assert_eq!(turn_7.geode, 0);

        let turn_11 = turn_7
            .get_state_with_built_robot(&blueprint, &ResourceType::Obsidian, 24)
            .unwrap();
        assert_eq!(turn_11.turn, 11);
        assert_eq!(turn_11.robots_ore, 1);
        assert_eq!(turn_11.robots_clay, 3);
        assert_eq!(turn_11.robots_obsidian, 1);
        assert_eq!(turn_11.robots_geode, 0);
        assert_eq!(turn_11.ore, 2);
        assert_eq!(turn_11.clay, 4);
        assert_eq!(turn_11.obsidian, 0);
        assert_eq!(turn_11.geode, 0);

        let turn_12 = turn_11
            .get_state_with_built_robot(&blueprint, &ResourceType::Clay, 24)
            .unwrap();
        assert_eq!(turn_12.turn, 12);
        assert_eq!(turn_12.robots_ore, 1);
        assert_eq!(turn_12.robots_clay, 4);
        assert_eq!(turn_12.robots_obsidian, 1);
        assert_eq!(turn_12.robots_geode, 0);
        assert_eq!(turn_12.ore, 1);
        assert_eq!(turn_12.clay, 7);
        assert_eq!(turn_12.obsidian, 1);
        assert_eq!(turn_12.geode, 0);

        let turn_15 = turn_12
            .get_state_with_built_robot(&blueprint, &ResourceType::Obsidian, 24)
            .unwrap();
        assert_eq!(turn_15.turn, 15);
        assert_eq!(turn_15.robots_ore, 1);
        assert_eq!(turn_15.robots_clay, 4);
        assert_eq!(turn_15.robots_obsidian, 2);
        assert_eq!(turn_15.robots_geode, 0);
        assert_eq!(turn_15.ore, 1);
        assert_eq!(turn_15.clay, 5);
        assert_eq!(turn_15.obsidian, 4);
        assert_eq!(turn_15.geode, 0);

        let turn_18 = turn_15
            .get_state_with_built_robot(&blueprint, &ResourceType::Geode, 24)
            .unwrap();
        assert_eq!(turn_18.turn, 18);
        assert_eq!(turn_18.robots_ore, 1);
        assert_eq!(turn_18.robots_clay, 4);
        assert_eq!(turn_18.robots_obsidian, 2);
        assert_eq!(turn_18.robots_geode, 1);
        assert_eq!(turn_18.ore, 2);
        assert_eq!(turn_18.clay, 17);
        assert_eq!(turn_18.obsidian, 3);
        assert_eq!(turn_18.geode, 0);

        let turn_21 = turn_18
            .get_state_with_built_robot(&blueprint, &ResourceType::Geode, 24)
            .unwrap();
        assert_eq!(turn_21.turn, 21);
        assert_eq!(turn_21.robots_ore, 1);
        assert_eq!(turn_21.robots_clay, 4);
        assert_eq!(turn_21.robots_obsidian, 2);
        assert_eq!(turn_21.robots_geode, 2);
        assert_eq!(turn_21.ore, 3);
        assert_eq!(turn_21.clay, 29);
        assert_eq!(turn_21.obsidian, 2);
        assert_eq!(turn_21.geode, 3);

        let turn_24 = turn_21.get_state_after_n_turns(3);
        assert_eq!(turn_24.turn, 24);
        assert_eq!(turn_24.robots_ore, 1);
        assert_eq!(turn_24.robots_clay, 4);
        assert_eq!(turn_24.robots_obsidian, 2);
        assert_eq!(turn_24.robots_geode, 2);
        assert_eq!(turn_24.ore, 6);
        assert_eq!(turn_24.clay, 41);
        assert_eq!(turn_24.obsidian, 8);
        assert_eq!(turn_24.geode, 9);
    }

    #[test]
    fn test_can_afford_in_future_with_given_input() {
        let blueprint = get_input("./src/day_19/input_test_given_input.txt")[7].clone();
        let state = State::new();
        assert_eq!(state.turn, 0);

        // dbg!(state.elapse_n_turns(4));
        let turn_5 = state
            .get_state_with_built_robot(&blueprint, &ResourceType::Ore, 24)
            .unwrap();
        assert_eq!(turn_5.turn, 5);
        assert_eq!(turn_5.robots_ore, 2);
        assert_eq!(turn_5.robots_clay, 0);
        assert_eq!(turn_5.robots_obsidian, 0);
        assert_eq!(turn_5.robots_geode, 0);
        assert_eq!(turn_5.ore, 1);
        assert_eq!(turn_5.clay, 0);
        assert_eq!(turn_5.obsidian, 0);
        assert_eq!(turn_5.geode, 0);

        let turn_9 = turn_5
            .get_state_with_built_robot(&blueprint, &ResourceType::Clay, 24)
            .unwrap();
        assert_eq!(turn_9.turn, 7);
        assert_eq!(turn_9.robots_ore, 2);
        assert_eq!(turn_9.robots_clay, 1);
        assert_eq!(turn_9.robots_obsidian, 0);
        assert_eq!(turn_9.robots_geode, 0);
        assert_eq!(turn_9.ore, 2);
        assert_eq!(turn_9.clay, 0);
        assert_eq!(turn_9.obsidian, 0);
        assert_eq!(turn_9.geode, 0);

        let turn_9 = turn_9
            .get_state_with_built_robot(&blueprint, &ResourceType::Clay, 24)
            .unwrap();
        assert_eq!(turn_9.turn, 9);
        assert_eq!(turn_9.robots_ore, 2);
        assert_eq!(turn_9.robots_clay, 2);
        assert_eq!(turn_9.robots_obsidian, 0);
        assert_eq!(turn_9.robots_geode, 0);
        assert_eq!(turn_9.ore, 3);
        assert_eq!(turn_9.clay, 2);
        assert_eq!(turn_9.obsidian, 0);
        assert_eq!(turn_9.geode, 0);

        let turn_10 = turn_9
            .get_state_with_built_robot(&blueprint, &ResourceType::Clay, 24)
            .unwrap();
        assert_eq!(turn_10.turn, 10);
        assert_eq!(turn_10.robots_ore, 2);
        assert_eq!(turn_10.robots_clay, 3);
        assert_eq!(turn_10.robots_obsidian, 0);
        assert_eq!(turn_10.robots_geode, 0);
        assert_eq!(turn_10.ore, 2);
        assert_eq!(turn_10.clay, 4);
        assert_eq!(turn_10.obsidian, 0);
        assert_eq!(turn_10.geode, 0);

        let turn_13 = turn_10
            .get_state_with_built_robot(&blueprint, &ResourceType::Obsidian, 24)
            .unwrap();
        assert_eq!(turn_13.turn, 13);
        assert_eq!(turn_13.robots_ore, 2);
        assert_eq!(turn_13.robots_clay, 3);
        assert_eq!(turn_13.robots_obsidian, 1);
        assert_eq!(turn_13.robots_geode, 0);
        assert_eq!(turn_13.ore, 6);
        assert_eq!(turn_13.clay, 3);
        assert_eq!(turn_13.obsidian, 0);
        assert_eq!(turn_13.geode, 0);

        let turn_14 = turn_13
            .get_state_with_built_robot(&blueprint, &ResourceType::Clay, 24)
            .unwrap();
        assert_eq!(turn_14.turn, 14);
        assert_eq!(turn_14.robots_ore, 2);
        assert_eq!(turn_14.robots_clay, 4);
        assert_eq!(turn_14.robots_obsidian, 1);
        assert_eq!(turn_14.robots_geode, 0);
        assert_eq!(turn_14.ore, 5);
        assert_eq!(turn_14.clay, 6);
        assert_eq!(turn_14.obsidian, 1);
        assert_eq!(turn_14.geode, 0);

        let turn_15 = turn_14
            .get_state_with_built_robot(&blueprint, &ResourceType::Clay, 24)
            .unwrap();
        assert_eq!(turn_15.turn, 15);
        assert_eq!(turn_15.robots_ore, 2);
        assert_eq!(turn_15.robots_clay, 5);
        assert_eq!(turn_15.robots_obsidian, 1);
        assert_eq!(turn_15.robots_geode, 0);
        assert_eq!(turn_15.ore, 4);
        assert_eq!(turn_15.clay, 10);
        assert_eq!(turn_15.obsidian, 2);
        assert_eq!(turn_15.geode, 0);

        let turn_16 = turn_15
            .get_state_with_built_robot(&blueprint, &ResourceType::Obsidian, 24)
            .unwrap();
        assert_eq!(turn_16.turn, 16);
        assert_eq!(turn_16.robots_ore, 2);
        assert_eq!(turn_16.robots_clay, 5);
        assert_eq!(turn_16.robots_obsidian, 2);
        assert_eq!(turn_16.robots_geode, 0);
        assert_eq!(turn_16.ore, 4);
        assert_eq!(turn_16.clay, 5);
        assert_eq!(turn_16.obsidian, 3);
        assert_eq!(turn_16.geode, 0);

        let turn_18 = turn_16
            .get_state_with_built_robot(&blueprint, &ResourceType::Obsidian, 24)
            .unwrap();
        assert_eq!(turn_18.turn, 18);
        assert_eq!(turn_18.robots_ore, 2);
        assert_eq!(turn_18.robots_clay, 5);
        assert_eq!(turn_18.robots_obsidian, 3);
        assert_eq!(turn_18.robots_geode, 0);
        assert_eq!(turn_18.ore, 6);
        assert_eq!(turn_18.clay, 5);
        assert_eq!(turn_18.obsidian, 7);
        assert_eq!(turn_18.geode, 0);

        let turn_20 = turn_18
            .get_state_with_built_robot(&blueprint, &ResourceType::Geode, 24)
            .unwrap();
        assert_eq!(turn_20.turn, 20);
        assert_eq!(turn_20.robots_ore, 2);
        assert_eq!(turn_20.robots_clay, 5);
        assert_eq!(turn_20.robots_obsidian, 3);
        assert_eq!(turn_20.robots_geode, 1);
        assert_eq!(turn_20.ore, 6);
        assert_eq!(turn_20.clay, 15);
        assert_eq!(turn_20.obsidian, 3);
        assert_eq!(turn_20.geode, 0);

        let turn_21 = turn_20
            .get_state_with_built_robot(&blueprint, &ResourceType::Obsidian, 24)
            .unwrap();
        assert_eq!(turn_21.turn, 21);
        assert_eq!(turn_21.robots_ore, 2);
        assert_eq!(turn_21.robots_clay, 5);
        assert_eq!(turn_21.robots_obsidian, 4);
        assert_eq!(turn_21.robots_geode, 1);
        assert_eq!(turn_21.ore, 6);
        assert_eq!(turn_21.clay, 10);
        assert_eq!(turn_21.obsidian, 6);
        assert_eq!(turn_21.geode, 1);

        let turn_23 = turn_21
            .get_state_with_built_robot(&blueprint, &ResourceType::Geode, 24)
            .unwrap();
        assert_eq!(turn_23.turn, 23);
        assert_eq!(turn_23.robots_ore, 2);
        assert_eq!(turn_23.robots_clay, 5);
        assert_eq!(turn_23.robots_obsidian, 4);
        assert_eq!(turn_23.robots_geode, 2);
        assert_eq!(turn_23.ore, 6);
        assert_eq!(turn_23.clay, 20);
        assert_eq!(turn_23.obsidian, 4);
        assert_eq!(turn_23.geode, 3);
    }
}
