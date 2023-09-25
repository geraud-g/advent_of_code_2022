use advent_of_code::utils::inputs::get_file;
use ahash::AHashMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::max;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

lazy_static! {
    static ref RE_PARSE_LINE: Regex = Regex::new(
        r"^Valve (\w+) has flow rate=(\d+); tunnel(s)? lead(s)? to valve(s)? ((?:\w+, )*\w+)$"
    )
    .unwrap();
    static ref START_IDX: usize = str_to_usize("AA").unwrap();
}

/// The path from one cave to another
#[derive(Debug, Copy, Clone)]
struct Path {
    distance: usize,
    dst_name: usize,
}

#[derive(Debug, Clone)]
struct Cave {
    idx: usize,
    flow_rate: usize,
    paths: Vec<Path>,
}

impl Default for Cave {
    fn default() -> Self {
        Self {
            idx: usize::MAX,
            flow_rate: 0,
            paths: vec![],
        }
    }
}

pub fn day_16() {
    // ========================================================================
    // Part 1
    let caves = get_input();
    let opened_valves = 0;
    let mut cache = AHashMap::with_capacity(45_000_000);
    let solution_1 = explore_caves_cached(&mut cache, &caves, *START_IDX, 30, opened_valves, 0);
    println!("\t- Solution 1 is : {}", solution_1);

    // ========================================================================
    // Part 2
    let solution_2 = part_two_init(&mut cache, &caves);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn get_input() -> Vec<Cave> {
    // ========================================================================
    // Get caves raw values
    let raw_caves: HashMap<usize, Cave> = get_file("./src/day_16/input.txt")
        .lines()
        .map(parse_line)
        .collect();

    // ========================================================================
    // Reduce caves paths.
    // We discard the caves containing 0 pressure, and update the path length accordingly
    let reduced_caves = reduce_caves_paths(&raw_caves);

    // ========================================================================
    // Reduce caves idx
    // This allows to access a cave/valve by using it's idx as an index on a small vector
    let mut idx_correspondence_table = HashMap::new();
    idx_correspondence_table.insert(0, 0);
    let mut current_idx = 1;
    for path_name in reduced_caves.keys() {
        if *path_name == 0 {
            continue;
        }
        if !idx_correspondence_table.contains_key(path_name) {
            idx_correspondence_table.insert(*path_name, current_idx);
            current_idx += 1;
        }
    }
    let valves_max_name_value = reduced_caves.len();
    let mut valves = vec![Cave::default(); valves_max_name_value];
    for (path_name, path) in &reduced_caves {
        let new_idx = idx_correspondence_table.get(path_name).unwrap();
        let mut new_valve = path.clone();
        new_valve.idx = *new_idx as usize;
        for child in new_valve.paths.iter_mut() {
            child.dst_name = *idx_correspondence_table.get(&child.dst_name).unwrap() as usize;
        }
        valves[*new_idx as usize] = new_valve;
    }
    valves
}

fn parse_line(line: &str) -> (usize, Cave) {
    let caps = RE_PARSE_LINE
        .captures(line)
        .unwrap_or_else(|| panic!("Cannot parse line {}", line));

    let name = caps[1].to_string();
    let flow_rate = caps[2].parse::<usize>().unwrap();
    let links: Vec<Path> = caps[6]
        .split(", ")
        .map(|s| Path {
            distance: 1,
            dst_name: str_to_usize(s).unwrap(),
        })
        .collect();

    (
        str_to_usize(&name).unwrap(),
        Cave {
            idx: str_to_usize(&name).unwrap(),
            flow_rate,
            paths: links,
        },
    )
}

fn str_to_usize(input: &str) -> Option<usize> {
    let bytes = input.as_bytes();
    if bytes.len() == 2 {
        let a = bytes[0] as usize - 65;
        let b = bytes[1] as usize - 65;

        Some((a << 5) | b)
    } else {
        None
    }
}

fn reduce_caves_paths(paths: &HashMap<usize, Cave>) -> HashMap<usize, Cave> {
    let valves: Vec<_> = paths
        .values()
        .filter(|&v| v.flow_rate > 0 || v.idx.eq(&START_IDX))
        .map(|v| v.idx)
        .collect();
    let reduced_paths: HashMap<_, _> = valves
        .par_iter()
        .map(|valve| {
            let reduced_path = reduce_cave_links(paths, *valve);
            (*valve, reduced_path)
        })
        .collect();
    reduced_paths
}

fn reduce_cave_links(paths: &HashMap<usize, Cave>, cave_idx: usize) -> Cave {
    let came_from = bfs(paths, cave_idx);
    let positive_flow_rates: Vec<_> = came_from
        .keys()
        .filter(|&&k| paths.get(&k).unwrap().flow_rate > 0)
        .cloned()
        .collect();
    let mut valve = Cave {
        idx: cave_idx,
        flow_rate: paths.get(&cave_idx).unwrap().flow_rate,
        paths: vec![],
    };
    for positive_flow_rate in positive_flow_rates {
        if positive_flow_rate.eq(&cave_idx) {
            continue;
        }
        let dist_from_start = get_path_len(&came_from, cave_idx, positive_flow_rate);
        valve.paths.push(Path {
            distance: dist_from_start,
            dst_name: positive_flow_rate,
        })
    }
    valve
}

fn bfs(paths: &HashMap<usize, Cave>, start: usize) -> HashMap<usize, usize> {
    let mut frontier = VecDeque::new();
    let mut came_from = HashMap::new();
    frontier.push_back(start);

    while let Some(current_valve) = frontier.pop_front() {
        for child in &paths.get(&current_valve).unwrap().paths {
            if let Entry::Vacant(e) = came_from.entry(child.dst_name) {
                e.insert(current_valve);
                if paths.get(&child.dst_name).unwrap().flow_rate == 0 {
                    frontier.push_back(child.dst_name)
                }
            }
        }
    }
    came_from
}
pub fn get_path_len(came_from: &HashMap<usize, usize>, start: usize, end: usize) -> usize {
    let mut path_len = 0;
    let mut current = end;

    while current != start {
        path_len += 1;
        current = *came_from.get(&current).unwrap();
    }
    path_len
}

type CacheKeyType = (usize, usize, u16, usize);

fn explore_caves_cached(
    cache: &mut AHashMap<CacheKeyType, usize>,
    valves: &Vec<Cave>,
    node: usize,
    remaining_time: usize,
    opened_valves: u16,
    generated_pressure: usize,
) -> usize {
    let hash = (node, remaining_time, opened_valves, generated_pressure);
    if let Some(cached_value) = cache.get(&hash) {
        *cached_value
    } else {
        let res = explore_caves(
            cache,
            valves,
            node,
            remaining_time,
            opened_valves,
            generated_pressure,
        );
        cache.insert(hash, res);
        res
    }
}

fn explore_caves(
    cache: &mut AHashMap<CacheKeyType, usize>,
    caves: &Vec<Cave>,
    current_cave_idx: usize,
    remaining_time: usize,
    opened_valves: u16,
    generated_pressure: usize,
) -> usize {
    let mut max_pressure = max(generated_pressure * remaining_time, 0);

    if remaining_time < 2 {
        return max_pressure;
    }

    for child in &caves[current_cave_idx].paths {
        let distance_to_child = child.distance;
        let is_valve_open = is_set(opened_valves, child.dst_name as u8);

        // Move to the cave and open the valve
        if !is_valve_open && remaining_time >= (distance_to_child + 1) {
            let child_score = (generated_pressure * (distance_to_child + 1))
                + explore_caves_cached(
                    cache,
                    caves,
                    child.dst_name,
                    remaining_time - (distance_to_child + 1),
                    set_bit(opened_valves, child.dst_name as u8),
                    generated_pressure + caves[child.dst_name].flow_rate,
                );
            if child_score > max_pressure {
                max_pressure = child_score
            }
        }
        // Move to the cave and ignore the valve, either because it's open already,
        // or because it could be more interesting to save time to open another valve
        if (is_valve_open || caves[child.dst_name].flow_rate < 5)
            && remaining_time >= distance_to_child
        {
            let child_score = (generated_pressure * distance_to_child)
                + explore_caves_cached(
                    cache,
                    caves,
                    child.dst_name,
                    remaining_time - distance_to_child,
                    opened_valves,
                    generated_pressure,
                );

            if child_score > max_pressure {
                max_pressure = child_score
            }
        }
    }

    max_pressure
}

fn part_two_init(cache: &mut AHashMap<CacheKeyType, usize>, valves: &Vec<Cave>) -> usize {
    let f_valves: Vec<Cave> = valves
        .iter()
        .filter(|&v| v.idx != *START_IDX)
        .cloned()
        .collect();
    let valves_len = f_valves.len();
    let half_valves_len = valves_len / 2;
    let mut max_val = 0;

    for comb in f_valves.iter().combinations(half_valves_len) {
        // ME -------------------------------------------------------------
        let mut opened_valves_me = 0b1111111111111111;
        for c in &comb {
            opened_valves_me = clear_bit(opened_valves_me, c.idx as u8);
        }
        // OTHER -----------------------------------------------------------
        let opened_valves_other = !opened_valves_me;
        let my_score = explore_caves_cached(cache, valves, *START_IDX, 26, opened_valves_me, 0);
        let op_score = explore_caves_cached(cache, valves, *START_IDX, 26, opened_valves_other, 0);

        let total_score = my_score + op_score;
        max_val = max(max_val, total_score);
    }
    max_val
}

/// Sets the bit at the given position `n` to 1.
fn set_bit(value: u16, n: u8) -> u16 {
    value | (1 << n)
}

/// Clears the bit at the given position `n` to 0.
fn clear_bit(value: u16, n: u8) -> u16 {
    value & !(1 << n)
}

/// Checks if the bit at the given position `n` is set (1).
fn is_set(value: u16, n: u8) -> bool {
    (value & (1 << n)) != 0
}
