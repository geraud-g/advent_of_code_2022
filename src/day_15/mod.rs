use advent_of_code::utils::inputs::get_file;
use lazy_static::lazy_static;
use rayon::prelude::*;
use regex::Regex;
use std::cmp::{max, min};
use std::str::FromStr;
lazy_static! {
    static ref RE_PARSE_LINE: Regex = Regex::new(r"x=(-?\d+), y=(-?\d+)").unwrap();
}

pub fn day_15() {
    let sensors = get_input();

    let solution_1 = part_one(&sensors, 2000000);
    println!("\t- Solution 1 is : {}", solution_1);

    // let solution_2 = part_two(&inputs);
    // println!("\t- Solution 2 is : {}", solution_2);
}

#[derive(Debug)]
struct Point {
    y: i64,
    x: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i64 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

#[derive(Debug)]
struct Sensor {
    center: Point,
    beacon: Point,
    distance: i64,
}

fn get_input() -> Vec<Sensor> {
    get_file("./src/day_15/input.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Sensor {
    let mut points = RE_PARSE_LINE.captures_iter(line).map(|cap| {
        let x = i64::from_str(&cap[1]).unwrap();
        let y = i64::from_str(&cap[2]).unwrap();
        Point::new(x, y)
    });
    let center = points.next().unwrap();
    let beacon = points.next().unwrap();
    let distance = manhattan_distance(&center, &beacon);
    Sensor {
        center,
        beacon,
        distance,
    }
}

fn part_one(sensors: &[Sensor], target_y: i64) -> usize {
    let (min_x, max_x) = get_x_boundaries(sensors);
    (min_x..=max_x)
        .into_par_iter()
        .filter(|x| !can_contain_beacon(sensors, &Point::new(*x, target_y)))
        .count()
        - 1
}

fn can_contain_beacon(sensors: &[Sensor], point: &Point) -> bool {
    for sensor in sensors {
        if manhattan_distance(&sensor.center, point) <= sensor.distance {
            return false;
        }
    }
    true
}

fn get_x_boundaries(sensors: &[Sensor]) -> (i64, i64) {
    let mut min_x = i64::max_value();
    let mut max_x = i64::min_value();
    for sensor in sensors {
        min_x = min(min_x, sensor.center.x - sensor.distance);
        max_x = max(max_x, sensor.center.x + sensor.distance);
    }
    (min_x, max_x)
}

fn part_two(inputs: &[i64]) -> i64 {
    unimplemented!()
}
