use advent_of_code::utils::inputs::get_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;

lazy_static! {
    static ref RE_PARSE_LINE: Regex = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
}

type CustomRangeInt = i64;
type CustomRange = (CustomRangeInt, CustomRangeInt);

pub fn day_15() {
    let sensors = get_input();

    let solution_1 = part_one(&sensors, 2000000);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(&sensors);
    println!("\t- Solution 2 is : {}", solution_2);
}

#[derive(Debug)]
struct Point {
    y: CustomRangeInt,
    x: CustomRangeInt,
}

impl Point {
    fn new(x: CustomRangeInt, y: CustomRangeInt) -> Self {
        Self { x, y }
    }
}

fn manhattan_distance(p1: &Point, p2: &Point) -> CustomRangeInt {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

#[derive(Debug)]
struct Sensor {
    center: Point,
    distance: CustomRangeInt,
}

fn get_input() -> Vec<Sensor> {
    get_file("./src/day_15/input.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Sensor {
    let mut points = RE_PARSE_LINE.captures_iter(line).map(|cap| {
        let x = CustomRangeInt::from_str(&cap[1]).unwrap();
        let y = CustomRangeInt::from_str(&cap[2]).unwrap();
        Point::new(x, y)
    });
    let center = points.next().unwrap();
    let beacon = points.next().unwrap();
    let distance = manhattan_distance(&center, &beacon);
    Sensor { center, distance }
}

fn part_one(sensors: &[Sensor], target_y: CustomRangeInt) -> CustomRangeInt {
    let x_boundaries = get_x_boundaries(sensors);
    let y_boundaries = get_y_boundaries(sensors);

    let mut ground_map = HashMap::new();
    for sensor in sensors {
        populate_ground_map(&mut ground_map, sensor, x_boundaries, y_boundaries);
    }

    ground_map
        .get(&target_y)
        .expect("Y {target_y} not found in the ground map")
        .iter()
        .map(|(a, b)| (b - a).abs())
        .sum()
}

fn populate_ground_map(
    ground_map: &mut HashMap<CustomRangeInt, Vec<CustomRange>>,
    sensor: &Sensor,
    x_boundaries: CustomRange,
    y_boundaries: CustomRange,
) {
    for y_delta in 0..=sensor.distance {
        let current_y = sensor.center.y - (sensor.distance - y_delta);
        if current_y < y_boundaries.0 {
            continue;
        }
        let range_start = max(sensor.center.x - y_delta, x_boundaries.0);
        let range_end = min(sensor.center.x + y_delta, x_boundaries.1);
        ground_map.entry(current_y).or_insert_with(Vec::new);
        update_row(
            ground_map.get_mut(&current_y).unwrap(),
            range_start,
            range_end,
        );
    }
    for y_delta in (0..sensor.distance).rev() {
        let current_y = sensor.center.y + (sensor.distance - y_delta);
        if current_y > y_boundaries.1 {
            continue;
        }
        let range_start = max(sensor.center.x - y_delta, x_boundaries.0);
        let range_end = min(sensor.center.x + y_delta, x_boundaries.1);
        ground_map.entry(current_y).or_insert_with(Vec::new);
        update_row(
            ground_map.get_mut(&current_y).unwrap(),
            range_start,
            range_end,
        );
    }
}

fn update_row(row: &mut Vec<CustomRange>, range_start: CustomRangeInt, range_end: CustomRangeInt) {
    let mut new_range = (range_start, range_end);
    let mut i = 0;

    while i != row.len() {
        if new_range.1 < row[i].0 || new_range.0 > row[i].1 {
            // No overlap with the new_range, keep it as it is
            i += 1;
        } else {
            // Overlap with new_range, merge them
            new_range.0 = new_range.0.min(row[i].0);
            new_range.1 = new_range.1.max(row[i].1);
            row.remove(i);
        }
    }

    // Insert the new (or merged) range at the correct position
    let pos = row
        .iter()
        .position(|&r| r.0 > new_range.0)
        .unwrap_or(row.len());
    row.insert(pos, new_range);
}

fn get_x_boundaries(sensors: &[Sensor]) -> CustomRange {
    let mut min_x = CustomRangeInt::max_value();
    let mut max_x = CustomRangeInt::min_value();
    for sensor in sensors {
        min_x = min(min_x, sensor.center.x - sensor.distance);
        max_x = max(max_x, sensor.center.x + sensor.distance);
    }
    (min_x, max_x)
}

fn get_y_boundaries(sensors: &[Sensor]) -> CustomRange {
    let mut min_y = CustomRangeInt::max_value();
    let mut max_y = CustomRangeInt::min_value();
    for sensor in sensors {
        min_y = min(min_y, sensor.center.y - sensor.distance);
        max_y = max(max_y, sensor.center.y + sensor.distance);
    }
    (min_y, max_y)
}

fn part_two(sensors: &[Sensor]) -> CustomRangeInt {
    let x_boundaries = (0, 4000000);
    let y_boundaries = (0, 4000000);

    let mut ground_map = HashMap::new();
    for sensor in sensors {
        populate_ground_map(&mut ground_map, sensor, x_boundaries, y_boundaries);
    }
    let reference_row = vec![(0, 4000000)];
    for y in 0..4000000 {
        if let Some(value) = ground_map.get(&y) {
            if value != &reference_row {
                let x = value[0].1 + 1;
                return 4000000 * x + y;
            }
        }
    }
    panic!("Solution not found for Day 15 Part 2")
}
// cargo run 15  55.30s user 0.95s system 99% cpu 56.619 total
