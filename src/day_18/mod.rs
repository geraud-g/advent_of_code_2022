use advent_of_code::utils::inputs::get_file;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;

pub fn day_18() {
    let cubes = get_input();

    let solution_1 = part_one(&cubes);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(&cubes);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn get_input() -> Vec<Cube> {
    get_file("./src/day_18/input.txt")
        .lines()
        .map(|l| {
            let split = l
                .split(',')
                .filter_map(|part| part.trim().parse::<i32>().ok())
                .collect::<Vec<i32>>();
            Cube {
                x: split[0],
                y: split[1],
                z: split[2],
            }
        })
        .collect()
}

type AreaLimits = ((i32, i32), (i32, i32), (i32, i32));

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Default)]
struct Cube {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

struct CubeNeighbours {
    cube: Cube,
    index: i32,
}

impl Cube {
    fn neighbours(&self) -> CubeNeighbours {
        CubeNeighbours {
            cube: *self,
            index: 0,
        }
    }
}

impl Iterator for CubeNeighbours {
    type Item = Cube;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(Cube {
                x: self.cube.x - 1,
                ..self.cube
            }),
            1 => Some(Cube {
                x: self.cube.x + 1,
                ..self.cube
            }),
            2 => Some(Cube {
                y: self.cube.y - 1,
                ..self.cube
            }),
            3 => Some(Cube {
                y: self.cube.y + 1,
                ..self.cube
            }),
            4 => Some(Cube {
                z: self.cube.z - 1,
                ..self.cube
            }),
            5 => Some(Cube {
                z: self.cube.z + 1,
                ..self.cube
            }),
            _ => return None,
        };

        self.index += 1;
        result
    }
}

fn part_one(cubes: &[Cube]) -> usize {
    let total_sides = cubes.len() * 6;
    let set_cubes: HashSet<Cube> = HashSet::from_iter(cubes.iter().cloned());

    let covered_sides = cubes
        .iter()
        .flat_map(|cube| cube.neighbours())
        .filter(|cube| set_cubes.contains(cube))
        .count();
    total_sides - covered_sides
}

fn part_two(cubes: &[Cube]) -> usize {
    let set_cubes: HashSet<Cube> = HashSet::from_iter(cubes.iter().cloned());
    let limit_values = get_area_limit(cubes);
    let outside_cubes = bfs(&set_cubes, limit_values);

    cubes
        .iter()
        .flat_map(|cube| cube.neighbours())
        .filter(|cube| outside_cubes.contains(cube))
        .count()
}

fn get_area_limit(cubes: &[Cube]) -> AreaLimits {
    let mut limit_values = ((999, -999), (999, -999), (999, -999));
    for cube in cubes {
        for neighbour in cube.neighbours() {
            limit_values.0 .0 = min(neighbour.x, limit_values.0 .0);
            limit_values.0 .1 = max(neighbour.x, limit_values.0 .1);
            limit_values.1 .0 = min(neighbour.y, limit_values.1 .0);
            limit_values.1 .1 = max(neighbour.y, limit_values.1 .1);
            limit_values.2 .0 = min(neighbour.z, limit_values.2 .0);
            limit_values.2 .1 = max(neighbour.z, limit_values.2 .1);
        }
    }
    limit_values
}

fn bfs(set_cubes: &HashSet<Cube>, area_limit: AreaLimits) -> HashSet<Cube> {
    let mut frontier = VecDeque::new();
    let mut came_from = HashMap::new();
    let start = Cube {
        x: area_limit.0 .0,
        y: area_limit.1 .0,
        z: area_limit.2 .0,
    };
    frontier.push_back(start);
    while let Some(current_node) = frontier.pop_front() {
        for child_node in current_node.neighbours() {
            if !is_inside_limit(set_cubes, area_limit, &child_node) {
                continue;
            }
            came_from.entry(child_node).or_insert_with(|| {
                frontier.push_back(child_node);
                current_node
            });
        }
    }
    came_from.keys().cloned().collect()
}

fn is_inside_limit(set_cubes: &HashSet<Cube>, area_limit: AreaLimits, cube: &Cube) -> bool {
    if set_cubes.contains(cube) {
        return false;
    }
    cube.x >= area_limit.0 .0
        && cube.x <= area_limit.0 .1
        && cube.y >= area_limit.1 .0
        && cube.y <= area_limit.1 .1
        && cube.z >= area_limit.2 .0
        && cube.z <= area_limit.2 .1
}
