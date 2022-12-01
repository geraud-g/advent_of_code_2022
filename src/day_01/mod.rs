use std::cmp::Reverse;

use itertools::Itertools;

use advent_of_code::parse_input;
use advent_of_code::utils::inputs::{get_file, LINE_ENDING};

pub fn day_01() {
    let inputs = get_input();

    let solution_a = part_a(&inputs);
    println!("\t- Solution A is : {}", solution_a);

    let solution_b = part_b(&inputs);
    println!("\t- Solution B is : {}", solution_b);
}


fn get_input() -> Vec<u32> {
    let file = get_file("./src/day_01/input.txt");
    let split_separator = format!("{}{}", LINE_ENDING, LINE_ENDING);
    let mut elves_food = vec![];

    for chunk in file.split(&split_separator) {
        elves_food.push(
            chunk
                .lines()
                .map(|l| parse_input!(l, u32))
                .sum()
        );
    }
    elves_food
}


fn part_a(elves_food: &[u32]) -> &u32 {
    elves_food
        .iter()
        .max()
        .unwrap()
}


fn part_b(elves_food: &[u32]) -> u32 {
    elves_food
        .iter()
        .sorted_by_key(|w| Reverse(*w))
        .take(3)
        .sum()
}
