use advent_of_code::utils::inputs::get_file;
use ahash::AHashSet;
use lazy_static::lazy_static;
use std::cmp::{max, min};
use std::hash::{Hash, Hasher};
use std::mem::swap;
use std::ops::{Add, AddAssign};

lazy_static! {
    static ref SUROUNDING: [Point; 8] = [
        Point { y: -1, x: -1 },
        Point { y: -1, x: 0 },
        Point { y: -1, x: 1 },
        Point { y: 0, x: -1 },
        Point { y: 0, x: 1 },
        Point { y: 1, x: -1 },
        Point { y: 1, x: 0 },
        Point { y: 1, x: 1 },
    ];
    static ref NORTH_DIRECTION: [Point; 3] = [
        Point { y: -1, x: -1 },
        Point { y: -1, x: 0 },
        Point { y: -1, x: 1 }
    ];
    static ref SOUTH_DIRECTION: [Point; 3] = [
        Point { y: 1, x: -1 },
        Point { y: 1, x: 0 },
        Point { y: 1, x: 1 }
    ];
    static ref WEST_DIRECTION: [Point; 3] = [
        Point { y: -1, x: -1 },
        Point { y: 0, x: -1 },
        Point { y: 1, x: -1 }
    ];
    static ref EAST_DIRECTION: [Point; 3] = [
        Point { y: -1, x: 1 },
        Point { y: 0, x: 1 },
        Point { y: 1, x: 1 }
    ];
    static ref DIRECTIONS: [[Point; 3]; 4] = [
        *NORTH_DIRECTION,
        *SOUTH_DIRECTION,
        *WEST_DIRECTION,
        *EAST_DIRECTION
    ];
}

pub fn day_23() {
    let elves = get_input();
    let solution_1 = part_one(&elves);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(&elves);
    println!("\t- Solution 2 is : {}", solution_2);
}

#[derive(Debug, Copy, Clone)]
struct Elf {
    id: usize,
    position: Point,
}

impl Elf {
    fn move_to(&self, direction: &Point) -> Self {
        Self {
            id: self.id,
            position: self.position + *direction,
        }
    }
}

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl Eq for Elf {}

impl Hash for Elf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.position.hash(state);
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Point {
    y: i64,
    x: i64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            y: self.y + other.y,
            x: self.x + other.x,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        self.y += other.y;
        self.x += other.x;
    }
}

fn get_input() -> AHashSet<Elf> {
    let mut elves = AHashSet::new();
    let mut current_idx = 0;
    for (y, line) in get_file("./src/day_23/input.txt").lines().enumerate() {
        line.chars()
            .enumerate()
            .filter(|(_, char)| '#'.eq(char))
            .for_each(|(x, _)| {
                let elf = Elf {
                    id: current_idx,
                    position: Point {
                        y: y as i64,
                        x: x as i64,
                    },
                };
                elves.insert(elf);
                current_idx += 1;
            });
    }
    elves
}

fn part_one(elves: &AHashSet<Elf>) -> usize {
    let mut elves_next_moves: Vec<Option<Point>> = vec![];
    let mut conflicts = AHashSet::new();
    let mut elves = elves.clone();
    let mut elves_next_turn = elves.clone();

    for current_turn in 0..10 {
        elves_next_moves.clear();
        conflicts.clear();
        elves_next_turn.clear();

        calculate_elves_next_moves(&elves, &mut elves_next_moves, current_turn);
        calculate_conflicts(&elves_next_moves, &mut conflicts);
        update_elves_next_turn(&elves_next_moves, &elves, &mut elves_next_turn, &conflicts);
        swap(&mut elves_next_turn, &mut elves);
    }
    let (min_point, max_point) = get_boundaries(&elves);
    count_space(&elves, &min_point, &max_point)
}

fn part_two(elves: &AHashSet<Elf>) -> usize {
    let mut elves_next_moves: Vec<Option<Point>> = vec![];
    let mut conflicts = AHashSet::new();
    let mut elves = elves.clone();
    let mut elves_next_turn = elves.clone();

    for current_turn in 0..10000 {
        elves_next_moves.clear();
        conflicts.clear();
        elves_next_turn.clear();

        calculate_elves_next_moves(&elves, &mut elves_next_moves, current_turn);
        calculate_conflicts(&elves_next_moves, &mut conflicts);
        update_elves_next_turn(&elves_next_moves, &elves, &mut elves_next_turn, &conflicts);
        if elves_next_turn.eq(&elves) {
            return current_turn + 1;
        }
        swap(&mut elves_next_turn, &mut elves);
    }
    panic!("Cannot find a solution for part 2")
}

fn calculate_elves_next_moves(
    elves: &AHashSet<Elf>,
    elves_destinations: &mut Vec<Option<Point>>,
    current_turn: usize,
) {
    elves_destinations.resize(elves.len(), None);

    for elf in elves {
        if is_alone(elves, elf) {
            continue;
        }
        'dir: for direction_idx in 0..4 {
            let direction = &DIRECTIONS[(direction_idx + current_turn) % 4];
            if can_move_to_tile(elves, elf, direction) {
                let next_position = elf.position + direction[1];
                elves_destinations[elf.id] = Some(next_position);
                break 'dir;
            }
        }
    }
}

fn is_alone(elves: &AHashSet<Elf>, elf: &Elf) -> bool {
    SUROUNDING.iter().all(|d| !elves.contains(&elf.move_to(d)))
}

fn can_move_to_tile(elves: &AHashSet<Elf>, elf: &Elf, directions: &[Point; 3]) -> bool {
    !elves.contains(&(elf.move_to(&directions[0])))
        && !elves.contains(&(elf.move_to(&directions[1])))
        && !elves.contains(&(elf.move_to(&directions[2])))
}

fn calculate_conflicts(elves_next_moves: &[Option<Point>], conflicts: &mut AHashSet<Point>) {
    let mut future_destinations = AHashSet::new();

    for point in elves_next_moves.iter().filter_map(|&option| option) {
        if !future_destinations.insert(point) {
            conflicts.insert(point);
        }
    }
}

fn update_elves_next_turn(
    elves_next_moves: &[Option<Point>],
    elves: &AHashSet<Elf>,
    elves_next_turn: &mut AHashSet<Elf>,
    conflicts: &AHashSet<Point>,
) {
    for elf in elves {
        let mut elf_next_turn = *elf;
        if let Some(next_position) = elves_next_moves[elf.id] {
            if !conflicts.contains(&next_position) {
                elf_next_turn.position = next_position;
            }
        }
        elves_next_turn.insert(elf_next_turn);
    }
}

fn get_boundaries(elves: &AHashSet<Elf>) -> (Point, Point) {
    let (mut min_x, mut min_y) = (i64::MAX, i64::MAX);
    let (mut max_x, mut max_y) = (i64::MIN, i64::MIN);
    for elf in elves {
        min_x = min(min_x, elf.position.x);
        min_y = min(min_y, elf.position.y);
        max_x = max(max_x, elf.position.x);
        max_y = max(max_y, elf.position.y);
    }
    (Point { y: min_y, x: min_x }, Point { y: max_y, x: max_x })
}

fn _print_map(elves: &AHashSet<Elf>, min_points: &Point, max_points: &Point) {
    for y in min_points.y..=max_points.y {
        for x in min_points.x..=max_points.x {
            if elves.contains(&Elf {
                id: 0,
                position: Point { y, x },
            }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn count_space(elves: &AHashSet<Elf>, min_points: &Point, max_points: &Point) -> usize {
    ((max_points.y - min_points.y + 1) * (max_points.x - min_points.x + 1)) as usize - elves.len()
}
