use advent_of_code::utils::inputs::{get_file, LINE_ENDING};
use lazy_static::lazy_static;
use regex::Regex;

const CUBE_SIDE: usize = 50;
const WIDTH: usize = 150;
const HEIGHT: usize = 200;

lazy_static! {
    static ref RE_INSTRUCTIONS: Regex = Regex::new(r"(\d+|L|R)").unwrap();
}

pub fn day_22() {
    let (map, instructions) = get_input();

    let solution_1 = part_one(&map, &instructions);
    println!("\t- Solution 1 is : {}", solution_1);
}

fn get_input() -> (Map, Vec<Instruction>) {
    let file = get_file("./src/day_22/input.txt");
    let split_separator = format!("{}{}", LINE_ENDING, LINE_ENDING);
    let split = file.split(&split_separator).collect::<Vec<_>>();
    let (board_map_part, instructions_part) = (split[0], split[1]);
    (
        get_board_map(board_map_part),
        get_instructions(instructions_part),
    )
}

fn get_board_map(map_part: &str) -> Map {
    let mut rows = vec![];

    for line in map_part.lines() {
        let mut row = vec![];
        for c in line.chars() {
            let tile = match c {
                ' ' => Tile::Void,
                '#' => Tile::Wall,
                '.' => Tile::Path,
                _ => panic!("Cannot parse tile `{}`", c),
            };
            row.push(tile)
        }
        rows.push(row)
    }
    let max_row_len = rows.iter().map(|r| r.len()).max().unwrap();
    for line in rows.iter_mut() {
        while line.len() < max_row_len {
            line.push(Tile::Void);
        }
    }

    let tiles: Vec<_> = rows.into_iter().flatten().collect();

    Map {
        cube_side: CUBE_SIDE,
        width: WIDTH,
        height: HEIGHT,
        tiles,
    }
}

fn get_instructions(instructions_part: &str) -> Vec<Instruction> {
    RE_INSTRUCTIONS
        .captures_iter(instructions_part)
        .map(|cap| match &cap[1] {
            "R" => Instruction::Right,
            "L" => Instruction::Left,
            val => Instruction::Move(val.parse::<u8>().unwrap()),
        })
        .collect()
}

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug)]
struct Position {
    idx: usize,
    direction: Direction,
}

impl Position {
    fn new(map: &Map) -> Self {
        let idx = map.get_upper_leftmost_path();
        Position {
            idx,
            direction: Direction::Right,
        }
    }

    fn apply_instruction(&mut self, map: &Map, instruction: &Instruction) {
        match instruction {
            Instruction::Left => self.rotate_left(),
            Instruction::Right => self.rotate_right(),
            Instruction::Move(value) => self.move_forward_flat(map, *value),
        }
    }

    fn rotate_left(&mut self) {
        use Direction::*;
        self.direction = match self.direction {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }
    fn rotate_right(&mut self) {
        use Direction::*;
        self.direction = match self.direction {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn move_forward_flat(&mut self, map: &Map, value: u8) {
        let old_idx = self.idx;
        let mut step_nbr = value;
        let mut last_valid_idx = self.idx;
        while step_nbr > 0 {
            let next_idx = self.move_flat_surface(map);
            match map.tiles[next_idx] {
                Tile::Path => {
                    self.idx = next_idx;
                    last_valid_idx = self.idx;
                    step_nbr -= 1
                }
                Tile::Wall => {
                    self.idx = last_valid_idx;
                    break;
                }
                Tile::Void => self.idx = next_idx,
            }
        }
    }

    fn move_flat_surface(&self, map: &Map) -> usize {
        match self.direction {
            Direction::Up => {
                if self.idx < map.width {
                    self.idx + (map.width * map.height - map.width)
                } else {
                    self.idx - map.width
                }
            }
            Direction::Right => {
                if (self.idx + 1) % map.width == 0 {
                    self.idx + 1 - map.width
                } else {
                    self.idx + 1
                }
            }
            Direction::Down => {
                if self.idx >= map.width * (map.height - 1) {
                    self.idx - (map.width * map.height - map.width)
                } else {
                    self.idx + map.width
                }
            }
            Direction::Left => {
                if self.idx % map.width == 0 {
                    self.idx + map.width - 1
                } else {
                    self.idx - 1
                }
            }
        }
    }
}

fn print_map(position: &Position, map: &Map) {
    for (idx, tile) in map.tiles.iter().enumerate() {
        if position.idx == idx {
            match position.direction {
                Direction::Up => print!("^"),
                Direction::Right => print!(">"),
                Direction::Down => print!("V"),
                Direction::Left => print!("<"),
            }
        } else {
            match tile {
                Tile::Path => print!("."),
                Tile::Wall => print!("#"),
                Tile::Void => print!(" "),
            }
        }
        if (idx + 1) % map.width == 0 {
            println!()
        }
    }
    println!()
}

#[derive(Debug)]
enum Tile {
    Void,
    Wall,
    Path,
}

#[derive(Debug)]
struct Map {
    cube_side: usize,
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Map {
    fn get_upper_leftmost_path(&self) -> usize {
        use Tile::*;
        for (idx, tile) in self.tiles.iter().enumerate() {
            if matches!(tile, Path) {
                return idx;
            }
        }
        panic!("Cannot get any Path tile")
    }
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
    Move(u8),
}

fn part_one(map: &Map, instructions: &Vec<Instruction>) -> usize {
    use Direction::*;
    let mut position = Position::new(map);

    for instruction in instructions {
        position.apply_instruction(map, instruction);
    }
    let facing_value = match position.direction {
        Up => 3,
        Right => 0,
        Left => 2,
        Down => 1,
    };
    let y = position.idx / map.width + 1;
    let x = position.idx % map.width + 1;
    (y * 1000) + (x * 4) + facing_value
}
