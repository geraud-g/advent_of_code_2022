use advent_of_code::utils::inputs::get_file;

const ROCK_SHAPE_BAR_H: &[u8] = &[0b11110000];
const ROCK_SHAPE_PLUS: &[u8] = &[0b01000000, 0b11100000, 0b01000000];
const ROCK_SHAPE_CORNER: &[u8] = &[0b00100000, 0b00100000, 0b11100000];
const ROCK_SHAPE_BAR_V: &[u8] = &[0b10000000, 0b10000000, 0b10000000, 0b10000000];
const ROCK_SHAPE_SQUARE: &[u8] = &[0b11000000, 0b11000000];

const ROCK_SHAPES: [&[u8]; 5] = [
    ROCK_SHAPE_BAR_H,
    ROCK_SHAPE_PLUS,
    ROCK_SHAPE_CORNER,
    ROCK_SHAPE_BAR_V,
    ROCK_SHAPE_SQUARE,
];

const FULL_ROW: u8 = 0b11111111;
const ROW_LEFT_TILE: u8 = 0b10000000;
const ROW_RIGHT_TILE: u8 = 0b00000011; // The row is only 7 wide

pub fn day_17() {
    let directions = get_input();

    let solution_1 = simulate_falls(&directions, 2022);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = simulate_falls(&directions, 1000000000000);
    println!("\t- Solution 2 is : {}", solution_2);
}

#[derive(Debug)]
enum JetDirection {
    Left,
    Right,
}

fn get_input() -> Vec<JetDirection> {
    get_file("./src/day_17/input.txt")
        .trim()
        .chars()
        .map(|c| match c {
            '<' => JetDirection::Left,
            _ => JetDirection::Right,
        })
        .collect()
}

fn simulate_falls(jet_pattern: &[JetDirection], mut rocks_to_fall: usize) -> usize {
    let mut state = State::new();
    let mut jet_idx = 0;
    let mut additional_height = 0;

    while state.nbr_landed_rocks < rocks_to_fall {
        apply_jet_push(&jet_pattern[jet_idx % jet_pattern.len()], &mut state);
        if move_down_and_try_to_land_rock(&mut state, jet_idx % jet_pattern.len())
            && additional_height == 0
        {
            // If a pattern is detected:
            //  - reduce the rocks to simulate
            //  - calculate the height generated by the cycle, and add it to `additional_height`
            if let Some((height_per_cycle, rocks_per_cycle)) = detect_cycle(&state) {
                let remaining_rocks_to_fall = rocks_to_fall - state.nbr_landed_rocks;
                let remaining_loops_nbr = remaining_rocks_to_fall / rocks_per_cycle;
                rocks_to_fall -= remaining_loops_nbr * rocks_per_cycle;
                additional_height = height_per_cycle * remaining_loops_nbr;
            }
        }
        jet_idx += 1;
    }
    state.highest_point + additional_height
}

/// Try to detect if a pattern is occurring among the falling rocks.
/// Returns Some(height_per_cycle, rocks_per_cycle) if a pattern is detected
fn detect_cycle(state: &State) -> Option<(usize, usize)> {
    let latest_rock = &state.history[state.history.len() - 1];
    for (idx, rock) in state.history.iter().enumerate().skip(1) {
        if idx == state.history.len() - 1 {
            break;
        }
        if does_cycle_exist(
            &state.history,
            &state.history[state.history.len() - 1],
            &state.history[idx],
            state.history.len() - 1,
            idx,
        ) {
            let y_offset = latest_rock.y - rock.y;
            let idx_offset = state.history.len() - 1 - idx;
            return Some((y_offset, idx_offset));
        }
    }
    None
}

/// Checks if a cycle exists in the history of falling rocks.
///
/// This function takes a slice of `RockHistory` objects and indices for the most recent
/// and oldest rocks to compare. It then checks if a cycle exists by comparing the vertical
/// positions (`y` values) of the rocks at these indices.
fn does_cycle_exist(
    history: &[RockHistory],
    earliest_rock: &RockHistory,
    latest_rock: &RockHistory,
    earliest_rock_idx: usize,
    latest_rock_idx: usize,
) -> bool {
    let y_offset = earliest_rock.y - latest_rock.y;
    let mut current_earliest_index = earliest_rock_idx;
    let mut current_latest_index = latest_rock_idx;

    if latest_rock.y <= 1 {
        // We can't go back in history if it's the 1st occurrence
        return false;
    }

    while current_earliest_index > latest_rock_idx {
        if !history[current_earliest_index]
            .is_eq_with_y_offset(&history[current_latest_index], y_offset)
        {
            return false;
        }

        if current_latest_index == 0 {
            return current_earliest_index == latest_rock_idx + 1;
        }

        current_earliest_index -= 1;
        current_latest_index -= 1;
    }
    true
}

/// Move the current piece according to the direction of the jet
fn apply_jet_push(direction: &JetDirection, state: &mut State) {
    match direction {
        JetDirection::Right => {
            if !has_reached_max_right_position(state) && !will_collide_right(state) {
                state.move_rock_right()
            }
        }
        JetDirection::Left => {
            if !has_reached_max_left_position(state) && !will_collide_left(state) {
                state.move_rock_left()
            }
        }
    }
}

fn has_reached_max_left_position(state: &State) -> bool {
    ROCK_SHAPES[state.rock_shape_idx]
        .iter()
        .any(|row| (row >> state.rock_x) & ROW_LEFT_TILE != 0)
}

fn has_reached_max_right_position(state: &State) -> bool {
    ROCK_SHAPES[state.rock_shape_idx]
        .iter()
        .any(|row| (row >> state.rock_x) & ROW_RIGHT_TILE != 0)
}

fn will_collide_left(state: &State) -> bool {
    for (i, row) in ROCK_SHAPES[state.rock_shape_idx].iter().enumerate() {
        let idx = state.rock_y + ROCK_SHAPES[state.rock_shape_idx].len() - 1 - i;
        let current_row = state.map[idx];
        if ((row >> state.rock_x) << 1) & current_row != 0 {
            return true;
        }
    }
    false
}

fn will_collide_right(state: &State) -> bool {
    for (i, piece_row) in ROCK_SHAPES[state.rock_shape_idx].iter().enumerate() {
        let idx = state.rock_y + ROCK_SHAPES[state.rock_shape_idx].len() - 1 - i;
        let current_row = state.map[idx];
        if ((piece_row >> state.rock_x) >> 1) & current_row != 0 {
            return true;
        }
    }
    false
}

/// Move the rock one unit lower, if possible, or land it.
/// Returns `true` if the rock has landed, `false` otherwise`
fn move_down_and_try_to_land_rock(state: &mut State, jet_idx: usize) -> bool {
    if !will_collide_with_bottom(state) {
        state.move_rock_down();
        false
    } else {
        let rock_x = state.rock_x;
        let rock_type = state.rock_shape_idx;
        state.merge_rock_to_map();
        state.nbr_landed_rocks += 1;
        state.update_highest_point();
        let rock_y = state.highest_point;
        state.add_next_piece();
        state
            .history
            .push(RockHistory::new(rock_type, rock_y, rock_x, jet_idx));
        true
    }
}

fn will_collide_with_bottom(state: &State) -> bool {
    for (i, row) in ROCK_SHAPES[state.rock_shape_idx].iter().enumerate() {
        let idx = state.rock_y + ROCK_SHAPES[state.rock_shape_idx].len() - 1 - i - 1;
        let current_row = state.map[idx];
        if (row >> state.rock_x) & current_row != 0 {
            return true;
        }
    }
    false
}

#[derive(Debug)]
struct RockHistory {
    rock_type: usize,
    y: usize,
    x: usize,
    jet_idx: usize,
}

impl RockHistory {
    fn new(rock_type: usize, y: usize, x: usize, jet_idx: usize) -> Self {
        Self {
            rock_type,
            y,
            x,
            jet_idx,
        }
    }

    /// State if the `other` RockHistory is equal to `self`, modulo the `y_offset`.
    /// It helps checking if `other` could be part of a pattern belonging to `self`
    fn is_eq_with_y_offset(&self, other: &Self, y_offset: usize) -> bool {
        self.rock_type == other.rock_type
            && self.x == other.x
            && self.y == other.y + y_offset
            && self.jet_idx == other.jet_idx
    }
}

#[derive(Debug)]
struct State {
    map: Vec<u8>,
    rock_shape_idx: usize,
    rock_x: usize,
    rock_y: usize,
    highest_point: usize,
    nbr_landed_rocks: usize,
    history: Vec<RockHistory>,
}

impl State {
    fn new() -> Self {
        let mut new = Self {
            map: vec![FULL_ROW],
            rock_shape_idx: ROCK_SHAPES.len() - 1,
            rock_x: 2,
            rock_y: 0,
            highest_point: 0,
            nbr_landed_rocks: 0,
            history: vec![],
        };
        new.add_next_piece();
        new
    }

    /// - Update `piece_nbr` to the next piece
    /// - Update the `self.rock_y` so there is a distance of 3 spaces between the highest point and the new piece
    fn add_next_piece(&mut self) {
        self.rock_shape_idx = (self.rock_shape_idx + 1) % ROCK_SHAPES.len();
        self.rock_x = 2; // Set a distance of 2 from the left wall
        self.rock_y = self.highest_point + 1 + 3; // Set a space of 3 with the highest point

        // Then add rows so there is no part in the new piece which has no corresponding row in the map
        let highest_y = self.highest_point + 3 + ROCK_SHAPES[self.rock_shape_idx].len() + 1;
        if highest_y > self.map.len() {
            for _ in self.map.len()..=highest_y {
                self.map.push(0);
            }
        }
    }

    fn move_rock_left(&mut self) {
        self.rock_x -= 1;
    }

    fn move_rock_right(&mut self) {
        self.rock_x += 1;
    }

    fn move_rock_down(&mut self) {
        self.rock_y -= 1;
    }

    fn merge_rock_to_map(&mut self) {
        for (i, rock_row) in ROCK_SHAPES[self.rock_shape_idx].iter().enumerate() {
            let idx = self.rock_y + ROCK_SHAPES[self.rock_shape_idx].len() - 1 - i;
            self.map[idx] |= rock_row >> self.rock_x;
        }
    }

    fn update_highest_point(&mut self) {
        for (i, row) in self.map.iter().enumerate().rev() {
            if *row != 0 {
                self.highest_point = i;
                return;
            }
        }
        // Should not be possible, since at least the floor should be present
        panic!("Could not find highest point.")
    }

    fn _print_state(&self) {
        println!("{:?}", self);

        // Format the current map
        let mut lines = vec![];
        for &byte in self.map.iter() {
            let binary_str: String = format!("{:08b}", byte)
                .chars()
                .map(|c| if c == '0' { '.' } else { '▓' })
                .collect();
            lines.push(binary_str);
        }

        // Add the current falling piece
        for (i, piece_row) in ROCK_SHAPES[self.rock_shape_idx].iter().enumerate() {
            let idx = self.rock_y + ROCK_SHAPES[self.rock_shape_idx].len() - 1 - i;

            let piece = piece_row >> self.rock_x;
            if piece != 0 {
                let mut chars: Vec<char> = lines[idx].chars().collect();
                let o: Vec<char> = format!("{:08b}", piece).chars().collect();
                for (ii, b) in o.iter().enumerate() {
                    if b.eq(&'1') {
                        chars[ii] = '░';
                    }
                }
                lines[idx] = chars.into_iter().collect();
            }
        }

        for (idx, line) in lines.iter().enumerate().rev() {
            let padded_idx = format!("{:06}", idx);
            println!("{}: {}", padded_idx, line);
        }
        println!("-------------------------------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_max_left() {
        // |...@...|
        // |..@@@..|
        // |...@...|
        // |.......|
        // +-------+
        let mut state = State::new();
        state.rock_shape_idx = 1;
        state.rock_x = 2;
        state.rock_y = 2;
        assert!(!has_reached_max_left_position(&state));

        // |.@.....|
        // |@@@....|
        // |.@.....|
        // |.......|
        // +-------+
        let mut state = State::new();
        state.rock_shape_idx = 1;
        state.rock_x = 0;
        state.rock_y = 2;
        assert!(has_reached_max_left_position(&state));

        // |.....@.|
        // |....@@@|
        // |.....@.|
        // |.......|
        // +-------+
        let mut state = State::new();
        state.rock_shape_idx = 1;
        state.rock_x = 4;
        state.rock_y = 2;
        assert!(!has_reached_max_left_position(&state));
    }

    #[test]
    fn test_is_max_right() {
        // |...@...|
        // |..@@@..|
        // |...@...|
        // |.......|
        // +-------+
        let mut state = State::new();
        state.rock_shape_idx = 1;
        state.rock_x = 2;
        state.rock_y = 2;
        assert!(!has_reached_max_right_position(&state));

        // |.@.....|
        // |@@@....|
        // |.@.....|
        // |.......|
        // +-------+
        let mut state = State::new();
        state.rock_shape_idx = 1;
        state.rock_x = 0;
        state.rock_y = 2;
        assert!(!has_reached_max_right_position(&state));

        // |.....@.|
        // |....@@@|
        // |.....@.|
        // |.......|
        // +-------+
        let mut state = State::new();
        state.rock_shape_idx = 1;
        state.rock_x = 4;
        state.rock_y = 2;
        assert!(has_reached_max_right_position(&state));
    }

    #[test]
    fn test_is_collision_bottom() {
        // |...@...|
        // |..@@@#.|
        // |...@###|
        // |.....#.|
        // +-------+
        let mut state = State::new();
        state.map[1] = 0b00000100;
        state.map[2] = 0b00001110;
        state.map[3] = 0b00000100;
        state.map[4] = 0b00000000;
        state.rock_shape_idx = 1;
        state.rock_x = 2;
        state.rock_y = 2;
        state.highest_point = 3;
        assert!(will_collide_with_bottom(&state));

        // |..@....|
        // |.@@@.#.|
        // |..@.###|
        // |.....#.|
        // +-------+
        let mut state = State::new();
        state.map[1] = 0b00000100;
        state.map[2] = 0b00001110;
        state.map[3] = 0b00000100;
        state.map[4] = 0b00000000;

        state.rock_shape_idx = 1;
        state.rock_x = 1;
        state.rock_y = 2;
        state.highest_point = 3;
        assert!(!will_collide_with_bottom(&state));
    }

    #[test]
    fn test_detect_cycle_a() {
        let mut state = State::new();
        state.history = vec![
            RockHistory::new(0, 1, 1, 0),  // 0
            RockHistory::new(1, 4, 2, 0),  // 1
            RockHistory::new(2, 6, 3, 0),  // 2
            RockHistory::new(0, 7, 1, 0),  // 3
            RockHistory::new(1, 10, 2, 0), // 4
            RockHistory::new(2, 12, 3, 0), // 5
        ];
        assert!(detect_cycle(&state).is_some())
    }

    #[test]
    fn test_detect_cycle_b() {
        let mut state = State::new();
        state.history = vec![
            RockHistory::new(0, 1, 1, 0),  // 0
            RockHistory::new(1, 4, 2, 0),  // 1
            RockHistory::new(2, 6, 3, 0),  // 2
            RockHistory::new(0, 7, 1, 0),  // 3
            RockHistory::new(1, 10, 2, 0), // 4
        ];
        assert!(!detect_cycle(&state).is_some())
    }

    #[test]
    fn test_detect_cycle_c() {
        let mut state = State::new();
        state.history = vec![
            // (type, y, x),
            RockHistory::new(0, 1, 1, 0), // 0
        ];
        assert!(!detect_cycle(&state).is_some())
    }

    #[test]
    fn test_detect_cycle_d() {
        let mut state = State::new();
        state.history = vec![
            // (type, y, x),
            RockHistory::new(2, 0, 1, 0),  // 0
            RockHistory::new(0, 1, 1, 0),  // 0
            RockHistory::new(1, 4, 2, 0),  // 1
            RockHistory::new(2, 6, 3, 0),  // 2 <-
            RockHistory::new(0, 7, 1, 0),  // 3
            RockHistory::new(1, 10, 2, 0), // 4
            RockHistory::new(2, 12, 3, 0), // 5 <-
        ];
        let res = detect_cycle(&state);
        assert!(res.is_some());
        if let Some((height_per_cycle, rocks_per_cycle)) = res {
            assert_eq!(height_per_cycle, 6);
            assert_eq!(rocks_per_cycle, 3);
        };
    }

    #[test]
    fn test_detect_cycle_e() {
        let mut state = State::new();
        state.history = vec![
            // (type, y, x),
            RockHistory::new(2, 0, 1, 0),  // 0
            RockHistory::new(0, 1, 1, 0),  // 0
            RockHistory::new(1, 4, 2, 0),  // 1
            RockHistory::new(2, 6, 3, 0),  // 2
            RockHistory::new(1, 12, 4, 0), // 3 <-- Pattern end
            RockHistory::new(0, 13, 1, 0), // 4
            RockHistory::new(1, 16, 2, 0), // 5
            RockHistory::new(2, 18, 3, 0), // 6
            RockHistory::new(1, 24, 4, 0), // 7 <-- Pattern start
        ];
        let res = detect_cycle(&state);
        assert!(res.is_some());
        if let Some((height_per_cycle, rocks_per_cycle)) = res {
            assert_eq!(height_per_cycle, 12);
            assert_eq!(rocks_per_cycle, 4);
        };
    }
}
