use advent_of_code::utils::inputs::get_file;
use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};

pub fn day_12() {
    let elevation_map = get_input();

    let solution_1 = part_one(&elevation_map);
    println!("\t- Solution A is : {}", &solution_1);

    let solution_2 = part_two(&elevation_map);
    println!("\t- Solution B is : {}", &solution_2);
}

fn get_input() -> Vec<Vec<i8>> {
    get_file("./src/day_12/input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => 0,
                    'E' => 27,
                    'a'..='z' => (c as i8 - 'a' as i8) + 1,
                    _ => 0,
                })
                .collect()
        })
        .collect()
}

fn part_one(elevation_map: &[Vec<i8>]) -> usize {
    let start = get_coord_from_value(elevation_map, 0);
    let goal = get_coord_from_value(elevation_map, 27);
    bfs(elevation_map, &start, &goal).unwrap()
}

fn part_two(elevation_map: &[Vec<i8>]) -> usize {
    // Retrieve all points we can start from
    let mut start_points = vec![];
    for (y, row) in elevation_map.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == 1 {
                start_points.push((y, x));
            }
        }
    }

    let goal = get_coord_from_value(elevation_map, 27);
    start_points
        .par_iter()
        .filter_map(|start| bfs(elevation_map, start, &goal))
        .min()
        .unwrap()
}

fn bfs(elevation_map: &[Vec<i8>], start: &Point, goal: &Point) -> Option<usize> {
    let mut frontier = VecDeque::new();
    let mut came_from = HashMap::new();
    frontier.push_back(*start);

    while let Some(current_node) = frontier.pop_front() {
        for child_node in get_children(elevation_map, &current_node) {
            came_from.entry(child_node).or_insert_with(|| {
                frontier.push_back(child_node);
                current_node
            });
            if child_node == *goal {
                return Some(get_path_len(&came_from, start, goal));
            }
        }
        came_from.entry(current_node).or_insert_with(|| {
            frontier.push_back(current_node);
            current_node
        });
    }
    None
}

pub fn get_path_len(came_from: &HashMap<Point, Point>, start: &Point, end: &Point) -> usize {
    let mut path_len = 0;
    let mut current = end;

    while current != start {
        path_len += 1;
        current = came_from.get(current).unwrap();
    }
    path_len
}

type Point = (usize, usize);

fn get_coord_from_value(graph: &[Vec<i8>], value: i8) -> Point {
    for (y, row) in graph.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == value {
                return (y, x);
            }
        }
    }
    panic!("Value {} not found", value)
}

fn get_children(graph: &[Vec<i8>], point: &Point) -> Vec<Point> {
    let mut children = vec![];
    if point.0 > 0 {
        let new_point = (point.0 - 1, point.1);
        if get_value(graph, point) - get_value(graph, &new_point) >= -1 {
            children.push((point.0 - 1, point.1))
        }
    }
    if point.0 < graph.len() - 1 {
        let new_point = (point.0 + 1, point.1);
        if get_value(graph, point) - get_value(graph, &new_point) >= -1 {
            children.push((point.0 + 1, point.1))
        }
    }

    if point.1 > 0 {
        let new_point = (point.0, point.1 - 1);

        if get_value(graph, point) - get_value(graph, &new_point) >= -1 {
            children.push((point.0, point.1 - 1))
        }
    }
    if point.1 < graph[0].len() - 1 {
        let new_point = (point.0, point.1 + 1);
        if get_value(graph, point) - get_value(graph, &new_point) >= -1 {
            children.push((point.0, point.1 + 1))
        }
    }

    children
}

fn get_value(graph: &[Vec<i8>], point: &Point) -> i8 {
    graph[point.0][point.1]
}
