use advent_of_code::parse_input;
use advent_of_code::utils::inputs::get_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

lazy_static! {
    static ref RE_COLUMN_NUMBERS: Regex = Regex::new(r"(?m)^(?: (\d)  ?)+[\s\n]*$").unwrap();
    static ref RE_CRATE_MOVE: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    static ref RE_CRATES_POSITIONS_LINE: Regex =
        Regex::new(r"(?m)^(?:[\[ ]([A-Z ])[\] ] ?)+").unwrap();
    static ref RE_CRATES_POSITIONS_VALUES: Regex = Regex::new(r"[\[ ]([A-Z ])[\] ] ?").unwrap();
}

type Stack = VecDeque<char>;
type Move = (usize, usize, usize);

pub fn day_05() {
    let (stacks, moves) = get_input();
    let solution_2 = part_one(&stacks, &moves);
    println!("\t- Solution 1 is : {}", solution_2);

    let solution_2 = part_two(&stacks, &moves);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn get_input() -> (Vec<Stack>, Vec<Move>) {
    let file = get_file("./src/day_05/input.txt");
    let mut moves = vec![];
    let mut stacks = vec![VecDeque::default(); get_stacks_number(&file)];

    for crate_line_capture in RE_CRATES_POSITIONS_LINE.captures_iter(&file) {
        let crate_position_line = crate_line_capture.get(0).unwrap().as_str();
        for (i, crate_capture_value) in RE_CRATES_POSITIONS_VALUES
            .captures_iter(crate_position_line)
            .enumerate()
        {
            let crate_value = crate_capture_value
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap();
            if !crate_value.is_whitespace() {
                stacks[i].push_back(crate_value)
            }
        }
    }

    for val in RE_CRATE_MOVE.captures_iter(&file) {
        let num = parse_input!(val.get(1).unwrap().as_str(), usize);
        let a = parse_input!(val.get(2).unwrap().as_str(), usize) - 1;
        let b = parse_input!(val.get(3).unwrap().as_str(), usize) - 1;
        moves.push((num, a, b));
    }
    (stacks, moves)
}

fn get_stacks_number(file: &str) -> usize {
    match RE_COLUMN_NUMBERS.captures(file) {
        Some(val) => val.get(1).unwrap().as_str().parse().unwrap(),
        None => panic!("Cannot find stack number: {:?}", file),
    }
}

fn part_one(stacks: &[Stack], moves: &[Move]) -> String {
    let mut new_stacks = stacks.to_owned();

    for (crates_nbr, stack_src, stack_dst) in moves {
        for _ in 0..*crates_nbr {
            if let Some(val) = new_stacks[*stack_src].pop_front() {
                new_stacks[*stack_dst].push_front(val);
            }
        }
    }
    new_stacks.iter().filter_map(|stack| stack.get(0)).collect()
}

fn part_two(stacks: &[Stack], moves: &[Move]) -> String {
    let mut new_stacks = stacks.to_owned();
    for (crates_nbr, stack_src, stack_dst) in moves {
        let mut buffer = VecDeque::new();
        for _ in 0..*crates_nbr {
            if let Some(val) = new_stacks[*stack_src].pop_front() {
                buffer.push_back(val);
            }
        }
        for _ in 0..*crates_nbr {
            new_stacks[*stack_dst].push_front(buffer.pop_back().unwrap());
        }
    }
    new_stacks.iter().filter_map(|stack| stack.get(0)).collect()
}
