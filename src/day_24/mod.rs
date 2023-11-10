use advent_of_code::utils::inputs::get_file;
use ahash::AHashMap;
use std::collections::VecDeque;

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

type MinuteIdx = (usize, usize);

pub fn day_24() {
    let (map, map_states) = get_input();

    let solution_1 = part_one(&map, &map_states);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(&map, &map_states);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn part_one(map: &MapProperties, map_states: &MapStates) -> usize {
    let path = bfs(map, map_states, map.start, map.end, 0);
    path.len() - 1
}

fn part_two(map: &MapProperties, map_states: &MapStates) -> usize {
    // First trip
    let path = bfs(map, map_states, map.start, map.end, 0);
    let last_pos = path[path.len() - 1];
    // Going back
    let path = bfs(map, map_states, map.end, map.start, last_pos.0);
    let last_pos = path[path.len() - 1];
    // Second trip
    let path = bfs(map, map_states, map.start, map.end, last_pos.0);
    path[path.len() - 1].0
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct MapProperties {
    width: usize,
    height: usize,
    start: usize,
    end: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct MapStates {
    states: Vec<State>,
}

impl MapStates {
    fn get_state_at_minute(&self, minute: usize) -> &State {
        &self.states[minute % self.states.len()]
    }

    fn generate_states(&mut self, map: &MapProperties) {
        let first_turn = self.states[0].clone();
        for current_turn in 0..1_000_000 {
            let new_state = self.states[current_turn].get_state_next_minute(map);
            if first_turn.eq(&new_state) {
                break;
            }
            self.states.push(new_state);
        }
    }

    fn get_possible_expedition_moves(
        &self,
        map: &MapProperties,
        curr_minute: usize,
        curr_idx: usize,
    ) -> Vec<usize> {
        let state = self.get_state_at_minute(curr_minute + 1);
        let mut moves = Vec::with_capacity(5);

        for direction in DIRECTIONS.iter() {
            if let Some(tmp_idx) = State::move_expedition(map, direction, curr_idx) {
                if state.cells[tmp_idx].is_empty() {
                    moves.push(tmp_idx)
                }
            }
        }
        if state.cells[curr_idx].is_empty() {
            moves.push(curr_idx)
        }
        moves
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct State {
    cells: Vec<Vec<Direction>>,
}

impl State {
    fn new_empty(map: &MapProperties) -> Self {
        Self {
            cells: (0..(map.width * map.height)).map(|_| vec![]).collect(),
        }
    }

    fn push_value(&mut self, value: Direction, idx: usize) {
        self.cells[idx].push(value)
    }

    fn get_values(&self, idx: usize) -> &Vec<Direction> {
        &self.cells[idx]
    }

    fn get_state_next_minute(&self, map: &MapProperties) -> Self {
        let mut new_state = State::new_empty(map);
        for (idx, cell) in self.cells.iter().enumerate() {
            for direction in cell {
                let idx_next_turn = Self::get_blizzard_idx_next_minute(map, direction, idx);
                new_state.push_value(*direction, idx_next_turn);
            }
        }
        new_state
    }

    fn get_blizzard_idx_next_minute(
        map: &MapProperties,
        direction: &Direction,
        current_idx: usize,
    ) -> usize {
        match direction {
            Direction::Up => {
                if current_idx < map.width * 2 {
                    current_idx + (map.width * (map.height - 3))
                } else {
                    current_idx - map.width
                }
            }
            Direction::Right => {
                if (current_idx + 2) % map.width == 0 {
                    current_idx - (map.width - 3)
                } else {
                    current_idx + 1
                }
            }
            Direction::Down => {
                if current_idx >= map.width * (map.height - 2) {
                    current_idx - (map.width * (map.height - 3))
                } else {
                    current_idx + map.width
                }
            }
            Direction::Left => {
                if current_idx % map.width == 1 {
                    current_idx + (map.width - 3)
                } else {
                    current_idx - 1
                }
            }
        }
    }

    fn move_expedition(
        map: &MapProperties,
        direction: &Direction,
        current_idx: usize,
    ) -> Option<usize> {
        match direction {
            Direction::Up => {
                if !(current_idx < map.width * 2
                    && (map.start != current_idx.saturating_sub(map.width)))
                {
                    return Some(current_idx - map.width);
                }
            }
            Direction::Right => {
                if !((current_idx + 2) % map.width == 0
                    || (current_idx < map.width)
                    || (current_idx >= map.width * (map.height - 1)))
                {
                    return Some(current_idx + 1);
                }
            }
            Direction::Down => {
                if !(current_idx >= map.width * (map.height - 2)
                    && map.end != current_idx + map.width)
                {
                    return Some(current_idx + map.width);
                }
            }
            Direction::Left => {
                if !(current_idx % map.width == 1
                    || (current_idx < map.width)
                    || (current_idx >= map.width * (map.height - 1)))
                {
                    return Some(current_idx - 1);
                }
            }
        }
        None
    }
}

fn get_input() -> (MapProperties, MapStates) {
    let file = get_file("./src/day_24/input.txt");
    let lines: Vec<_> = file.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let start_x = lines[0].find('.').expect("Cannot find starting point");
    let end_x = lines[lines.len() - 1]
        .find('.')
        .expect("Cannot find starting point");
    let start = start_x;
    let end = width * height - (width - end_x);
    let map = MapProperties {
        width,
        height,
        start,
        end,
    };
    let mut state = State::new_empty(&map);

    for (y, row) in lines.iter().enumerate() {
        for (x, cell) in row.chars().enumerate() {
            match cell {
                '>' => {
                    state.push_value(Direction::Right, y * width + x);
                }
                '<' => {
                    state.push_value(Direction::Left, y * width + x);
                }
                'v' => {
                    state.push_value(Direction::Down, y * width + x);
                }
                '^' => {
                    state.push_value(Direction::Up, y * width + x);
                }
                _ => {}
            };
        }
    }
    let mut map_states = MapStates {
        states: vec![state],
    };
    map_states.generate_states(&map);
    (map, map_states)
}

#[allow(dead_code)]
fn print_map(map: &MapProperties, state: &State, expedition: usize) {
    for y in 0..map.height {
        for x in 0..map.width {
            let point = y * map.width + x;
            let point_value = state.get_values(point);
            if (y * map.width + x) == expedition {
                print!("E");
            } else if point.eq(&map.start) {
                print!("S")
            } else if point.eq(&map.end) {
                print!("G")
            } else if point_value.len() > 1 {
                print!("{}", point_value.len())
            } else if point_value.len() == 1 {
                match point_value[0] {
                    Direction::Up => print!("^"),
                    Direction::Right => print!(">"),
                    Direction::Down => print!("v"),
                    Direction::Left => print!("<"),
                }
            } else if y == 0 || y == map.height - 1 || x == 0 || x == map.width - 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}

fn bfs(
    map: &MapProperties,
    map_states: &MapStates,
    start: usize,
    end: usize,
    minute: usize,
) -> Vec<MinuteIdx> {
    let mut frontier = VecDeque::with_capacity(1000);
    let mut came_from = AHashMap::with_capacity(120_000);
    frontier.push_back((minute, start));

    while let Some((curr_minute, curr_idx)) = frontier.pop_front() {
        for possible_idx in map_states.get_possible_expedition_moves(map, curr_minute, curr_idx) {
            came_from
                .entry((curr_minute + 1, possible_idx))
                .or_insert_with(|| {
                    frontier.push_back((curr_minute + 1, possible_idx));
                    (curr_minute, curr_idx)
                });
            if possible_idx == end {
                return get_path(&came_from, start, end, curr_minute + 1, minute);
            }
        }
    }
    panic!("No path found")
}

fn get_path(
    came_from: &AHashMap<MinuteIdx, MinuteIdx>,
    start: usize,
    end: usize,
    minute: usize,
    start_min: usize,
) -> Vec<MinuteIdx> {
    let mut current = (minute, end);
    let mut path = vec![current];

    while current != (start_min, start) {
        current = *came_from.get(&current).unwrap();
        path.push(current);
    }
    path.reverse();
    path
}
