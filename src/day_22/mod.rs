use advent_of_code::utils::inputs::{get_file, LINE_ENDING};
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{max, min};
use crate::day_22::Tile::Void;

lazy_static! {
    static ref RE_INSTRUCTIONS: Regex = Regex::new(r"(\d+|L|R)").unwrap();
}

pub fn day_22() {
    // 27492
    // 78291
    // TODO It’s an other day where indexes should start at 1
    let (map, instructions) = get_input();

    let solution_1 = part_one(&map, &instructions);
    println!("\t- Solution 1 is : {}", solution_1);
    //
    // let solution_2 = part_two(&inputs);
    // println!("\t- Solution 2 is : {}", solution_2);
}

fn get_input() -> (Map, Vec<Instruction>) {
    let file = get_file("./src/day_22/input.txt");
    let split_separator = format!("{}{}", LINE_ENDING, LINE_ENDING);
    let split = file.split(&split_separator).collect::<Vec<_>>();
    let (map_part, instructions_part) = (split[0], split[1]);
    (get_map(map_part), get_instructions(instructions_part))
}

fn get_map(map_part: &str) -> Map {
    use Tile::*;
    let mut tiles = vec![];
    for line in map_part.lines() {
        let mut row = vec![Void];
        for c in line.chars() {
            let tile = match c {
                ' ' => Void,
                '#' => Wall,
                '.' => Path,
                _ => panic!("Cannot parse tile `{}`", c),
            };
            row.push(tile)
        }
        // The row len is not the same for all lines, so adding a Void here helps later
        // to detect the end a of a row
        // TODO : What if the position is going down ?
        // TODO : Just add Void until line == MAX_LEN
        // TODO : Do it in post process
        tiles.push(row)
    }
    let max_row_len = tiles.iter().map(|r|r.len()).max().unwrap();
    for line in tiles.iter_mut() {
        while line.len() < max_row_len + 1 {
            line.push(Void);
        }
    }
    let top_row = (0..max_row_len).into_iter().map(|_|Void).collect::<Vec<_>>();
    tiles.insert(0, top_row);
    let bottom_row = (0..max_row_len).into_iter().map(|_|Void).collect::<Vec<_>>();
    tiles.push(bottom_row);
    Map { tiles }
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
    x: usize,
    y: usize,
    direction: Direction,
}

impl Position {
    fn new(map: &Map) -> Self {
        let (x, y) = map.get_upper_leftmost_path();
        Position {
            x,
            y,
            direction: Direction::Right,
        }
    }

    fn apply_instruction(&mut self, map: &Map, instruction: &Instruction) {
        use Instruction::*;
        match instruction {
            Left => self.rotate_left(),
            Right => self.rotate_right(),
            Move(value) => self.move_forward(map, *value),
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

    fn move_forward(&mut self, map: &Map, value: u8) {
        let new_pos = map.move_from_to((self.x, self.y), &self.direction, value);
        self.x = new_pos.0;
        self.y = new_pos.1;
    }
}

#[derive(Debug)]
enum Tile {
    Void,
    Wall,
    Path,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn get_upper_leftmost_path(&self) -> (usize, usize) {
        use Tile::*;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if matches!(tile, Path) {
                    println!("Start is {:?}", (x, y));
                    return (x, y);
                }
            }
        }
        panic!("Cannot get any Path tile")
    }
    // TODO : Rename to "bounded_coordinates ?"
    fn move_from_to(
        &self,
        from: (usize, usize),
        direction: &Direction,
        steps: u8,
    ) -> (usize, usize) {
        use Direction::*;
        let mut new_coord = from;
        let mut tmp = new_coord;
        let action: Box<dyn Fn(&mut usize, &mut usize)> = match direction {
            Up => Box::new(|x: &mut usize, y: &mut usize| {
                *y = y.saturating_sub(1);
            }),
            Right => Box::new(|x: &mut usize, y: &mut usize| {
                *x = x.saturating_add(1);
            }),
            Down => Box::new(|x: &mut usize, y: &mut usize| {
                *y = y.saturating_add(1);
            }),
            Left => Box::new(|x: &mut usize, y: &mut usize| {
                *x = x.saturating_sub(1);
            }),
        };
        // TODO : If a movement instruction would take you off of the map,
        // TODO : you wrap around to the other side of the board. In other words,
        // TODO : if your next tile is off of the board, you should instead look in
        // TODO : the direction opposite of your current facing as far as you can until
        //TODO :  you find the opposite edge of the board, then reappear there.
        for _ in 0..steps {
            action(&mut tmp.0, &mut tmp.1);
            if matches!(self.tiles[tmp.1][tmp.0], Tile::Void) {
                self.wrap(&mut tmp.0, &mut tmp.1, direction);
            }
            // TODO: Concatenate wrap & action ?
            if matches!(self.tiles[tmp.1][tmp.0], Tile::Path) && tmp != new_coord {
                new_coord = tmp;
            } else {
                break;
            }
        }
        new_coord
    }
    fn wrap(&self, x: &mut usize, y: &mut usize, direction: &Direction) {
        use Direction::*;
        // TODO : Go to other side
        // TODO: Iter until `not Void`
        // TODO: If RIGHT: From 0 to LEN
        // TODO: If LEFT: From LEN to 0
        // TODO: Etc
        match direction {
            Up => (

                ),
            Right => (),
            Down => (),
            Left => (),
        }
        todo!()
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
        // println!(">>> Instruction is {:?}", instruction);
        position.apply_instruction(map, instruction);
        // println!("{:?}", position);
    }
    let facing_value = match position.direction {
        Up => 3,
        Right => 0,
        Left => 2,
        Down => 1,
    };
    println!("{} {} {}", position.y, position.x, facing_value);
    (position.x * 1000) + (position.x * 4) + facing_value
}
