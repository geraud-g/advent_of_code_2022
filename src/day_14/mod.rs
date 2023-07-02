use advent_of_code::utils::inputs::get_file;
use std::cmp::max;
use std::ops::RangeInclusive;
use std::str::FromStr;

pub fn day_14() {
    let mut cave = get_input(1);
    let solution_1 = part_one(&mut cave);
    println!("\t- Solution 1 is : {}", solution_1);

    let mut cave = get_input(2);
    let solution_2 = part_two(&mut cave);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn get_input(part: usize) -> Cave {
    let file = get_file("./src/day_14/input.txt");
    let mut walls = vec![];
    let (mut max_x, mut max_y) = (usize::MIN, usize::MIN);

    for line in file.lines() {
        let wall_coordinates =
            parse_wall_coordinates(line).expect("Cannot parse {line} coordinates");
        for (x, y) in &wall_coordinates {
            max_x = max(max_x, *x);
            max_y = max(max_y, *y);
        }
        walls.push(wall_coordinates);
    }
    if part == 2 {
        max_x = 1000;
        max_y += 2
    }
    let mut cave = Cave::new(vec![false; (max_x + 1) * (max_y + 1)], max_x, max_y);

    for wall_coordinates in &walls {
        for window in wall_coordinates.windows(2) {
            cave.add_wall(window[0].0, window[0].1, window[1].0, window[1].1);
        }
    }
    cave
}

pub fn parse_wall_coordinates(s: &str) -> Result<Vec<(usize, usize)>, std::num::ParseIntError> {
    s.split(" -> ")
        .map(|part| {
            let mut coords = part.split(',');
            let x = usize::from_str(coords.next().unwrap())?;
            let y = usize::from_str(coords.next().unwrap())?;
            Ok((x, y))
        })
        .collect()
}

struct Cave {
    tiles: Vec<bool>,
    max_x: usize,
    max_y: usize,
    width: usize,
}

impl Cave {
    fn new(tiles: Vec<bool>, max_x: usize, max_y: usize) -> Self {
        Self {
            tiles,
            max_x,
            max_y,
            width: max_x + 1,
        }
    }

    fn add_wall(&mut self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) {
        // horizontal wall
        if start_x != end_x {
            for x in get_range(start_x, end_x) {
                let index = start_y * self.width + x;
                self.tiles[index] = true;
            }
        }
        // vertical wall
        else {
            for y in get_range(start_y, end_y) {
                let index = y * self.width + start_x;
                self.tiles[index] = true;
            }
        }
    }

    fn get_next_sand_drop_coordinates(&mut self) -> Option<usize> {
        if let Some(sand_coordinates) = self.get_sand_coordinates(500) {
            return Some(sand_coordinates);
        }
        None
    }
    fn get_sand_coordinates(&self, mut sand_coordinate: usize) -> Option<usize> {
        for _ in self.get_y(sand_coordinate)..self.max_y {
            let next_position = sand_coordinate + self.width;

            if self.tiles[next_position] {
                let bottom_left = sand_coordinate + self.max_x;
                if self.get_x(sand_coordinate) != 0 && !self.tiles[bottom_left] {
                    // Sand roll left
                    return self.get_sand_coordinates(bottom_left);
                }

                let bottom_right = sand_coordinate + self.max_x + 2;
                if self.get_x(sand_coordinate) < self.max_x && !self.tiles[bottom_right] {
                    // Sand roll right
                    return self.get_sand_coordinates(bottom_right);
                }
                // Sand land and stop
                return Some(sand_coordinate);
            }
            // Sand fall out of bound
            if self.get_y(sand_coordinate) > self.max_y {
                return None;
            }
            sand_coordinate += self.width
        }
        None
    }

    fn get_y(&self, coordinate: usize) -> usize {
        coordinate / self.width
    }

    fn get_x(&self, coordinate: usize) -> usize {
        coordinate % self.width
    }
}

fn get_range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

fn part_one(cave: &mut Cave) -> u32 {
    for count in 0..1_000_000 {
        if let Some(sand_coordinate) = cave.get_next_sand_drop_coordinates() {
            cave.tiles[sand_coordinate] = true;
        } else {
            return count;
        }
    }
    panic!("Did not finished sand simulation")
}

fn part_two(cave: &mut Cave) -> u32 {
    cave.add_wall(0, cave.max_y, cave.max_x, cave.max_y);
    for count in 0..1_000_000 {
        if let Some(sand_coordinate) = cave.get_next_sand_drop_coordinates() {
            cave.tiles[sand_coordinate] = true;
            if sand_coordinate == 500 {
                return count + 1;
            }
        }
    }
    panic!("Did not finished sand simulation")
}
