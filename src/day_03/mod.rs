use advent_of_code::utils::inputs::get_file;
use std::collections::HashSet;

pub fn day_03() {
    let rucksacks = get_input();

    let solution_1 = part_one(&rucksacks);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(&rucksacks);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn get_input() -> Vec<Vec<char>> {
    get_file("./src/day_03/input.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect()
}

fn part_one(rucksacks: &[Vec<char>]) -> i32 {
    rucksacks
        .iter()
        .map(|rucksack| get_rucksack_priority(rucksack))
        .sum()
}

fn get_rucksack_priority(rucksack: &[char]) -> i32 {
    let compartment_size = rucksack.len() / 2;
    let left_part: HashSet<_> = rucksack[0..compartment_size].iter().collect();
    let right_part: HashSet<_> = rucksack[compartment_size..].iter().collect();
    let common = left_part.intersection(&right_part).next().unwrap();
    get_char_priority(common)
}

fn get_char_priority(ch: &char) -> i32 {
    let value = *ch as i32;
    if value >= 97 {
        value - 96
    } else {
        value - 38
    }
}

fn part_two(rucksacks: &[Vec<char>]) -> i32 {
    let mut item_types = vec![];
    let hashset_rucksacks: Vec<HashSet<_>> = rucksacks.iter().map(|r| r.iter().collect()).collect();

    for rucksacks_chunk in hashset_rucksacks.chunks(3) {
        let first_intersection: HashSet<_> = rucksacks_chunk[0]
            .intersection(&rucksacks_chunk[1])
            .map(|c| c.to_owned())
            .collect();
        let total_intersection = first_intersection.intersection(&rucksacks_chunk[2]);
        item_types.push(get_char_priority(
            total_intersection.into_iter().next().unwrap(),
        ))
    }
    item_types.iter().sum()
}
