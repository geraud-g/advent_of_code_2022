use itertools::Itertools;
use advent_of_code::utils::inputs::get_file;


pub fn day_06() {
    let datastream_buffer = get_input();

    let solution_a = part_a(&datastream_buffer);
    println!("\t- Solution A is : {}", solution_a);

    let solution_b = part_b(&datastream_buffer);
    println!("\t- Solution B is : {}", solution_b);
}


fn get_input() -> String {
    get_file("./src/day_06/input.txt")
}


fn part_a(datastream_buffer: &str) -> usize {
    let consecutive_characters = 4;
    get_consecutive_distinct_characters_idx(datastream_buffer, consecutive_characters) + consecutive_characters
}


fn part_b(datastream_buffer: &str) -> usize {
    let consecutive_characters = 14;
    get_consecutive_distinct_characters_idx(datastream_buffer, consecutive_characters) + consecutive_characters
}


fn get_consecutive_distinct_characters_idx(text: &str, char_nbr: usize) -> usize {
    for (idx, window) in text.chars().collect_vec().windows(char_nbr).enumerate() {
        if window.iter().unique().count() == char_nbr {
            return idx;
        }
    }
    panic!("Could not find {} consecutive characters for text `{}`", char_nbr, text)
}
