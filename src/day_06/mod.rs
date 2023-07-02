use advent_of_code::utils::inputs::get_file;
use itertools::Itertools;

pub fn day_06() {
    let datastream_buffer = get_input();

    let solution_1 = part_one(&datastream_buffer);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(&datastream_buffer);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn get_input() -> String {
    get_file("./src/day_06/input.txt")
}

fn part_one(datastream_buffer: &str) -> usize {
    let consecutive_characters = 4;
    get_consecutive_distinct_characters_idx(datastream_buffer, consecutive_characters)
        + consecutive_characters
}

fn part_two(datastream_buffer: &str) -> usize {
    let consecutive_characters = 14;
    get_consecutive_distinct_characters_idx(datastream_buffer, consecutive_characters)
        + consecutive_characters
}

fn get_consecutive_distinct_characters_idx(text: &str, char_nbr: usize) -> usize {
    for (idx, window) in text.chars().collect_vec().windows(char_nbr).enumerate() {
        if window.iter().unique().count() == char_nbr {
            return idx;
        }
    }
    panic!(
        "Could not find {} consecutive characters for text `{}`",
        char_nbr, text
    )
}
