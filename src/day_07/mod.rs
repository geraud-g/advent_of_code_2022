use advent_of_code::utils::inputs::get_file;
use advent_of_code::utils::arena_tree::{ArenaTree, Node};

pub fn day_07() {
    let commands = get_input();

    // let solution_a = part_a(&inputs);
    // println!("\t- Solution A is : {}", solution_a);
    //
    // let solution_b = part_b(&inputs);
    // println!("\t- Solution B is : {}", solution_b);
}


fn get_input() -> usize {
    // let mut arena_tree = ArenaTree::default();
    for line in  get_file("./src/day_07/input.txt").lines() {
        if line.starts_with("$") {
            println!("Command: {:?}", line)
        } else {
            println!("Output: - {:?}", line)
        }
    }
    0
}


fn part_a(inputs: &[i32]) -> i32 {
    unimplemented!()
}


fn part_b(inputs: &[i32]) -> i32 {
    unimplemented!()
}


enum  Command {
    Cd(String),
    Ls(String)
}


struct File {
    name: String,
    is_directory: bool,
    size: usize
}
