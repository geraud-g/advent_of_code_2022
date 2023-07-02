use itertools::Itertools;
use std::cmp::Reverse;

use advent_of_code::parse_input;
use advent_of_code::utils::inputs::{get_file, LINE_ENDING};

pub fn day_01() {
    let inputs = get_input();

    let solution_1 = part_one(&inputs);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(&inputs);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn get_input() -> Vec<u32> {
    let file = get_file("./src/day_01/input.txt");
    let split_separator = format!("{}{}", LINE_ENDING, LINE_ENDING);
    file.split(&split_separator)
        .map(|chunk| chunk.lines().map(|l| parse_input!(l, u32)).sum())
        .collect()
}

fn part_one(elves_food: &[u32]) -> &u32 {
    elves_food.iter().max().unwrap()
}

fn part_two(elves_food: &[u32]) -> u32 {
    elves_food
        .iter()
        .sorted_by_key(|w| Reverse(*w))
        .take(3)
        .sum()
}
