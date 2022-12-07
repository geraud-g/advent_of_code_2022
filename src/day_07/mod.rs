use regex::Regex;
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


fn get_input() -> ArenaTree<File> {
    let mut arena_tree = ArenaTree::default();
    let mut current_file = File {
        name: "/".to_string(),
        is_directory: true,
        size: 0,
    };
    let mut current_path_index = arena_tree.insert_node(current_file);
    let root_index = current_path_index;

    for chunk in get_file("./src/day_07/input.txt").split('$').into_iter() {
        let mut lines = chunk.lines();
        if let Some(raw_command) = lines.next() {
            if let Command::Cd(path) = get_command(raw_command) {
                match path.as_ref() {
                    ".." => { current_path_index = arena_tree.get_unwrapped(current_path_index).parent.unwrap() }
                    "/" => { current_path_index = root_index }
                    _ => {
                        // TODO : Check if path in child
                        // TODO : If so just update idx
                        // TODO : If not create node. add it, update IDX
                    }
                }
            } else {
                println!(">>> Ls");
                for x in lines {
                    println!(">>> - {}", x)
                }
                // TODO : ls
            }
        }
    }
    arena_tree
}

fn get_command(line: &str) -> Command {
    let RE_CD: Regex = Regex::new(r"(?m)^\s*cd\s+(.+)\s*").unwrap();
    if let Some(value) = RE_CD.captures(line) {
        Command::Cd(value.get(1).unwrap().as_str().to_string())
    } else {
        Command::Ls
    }
}


fn part_a(inputs: &[i32]) -> i32 {
    unimplemented!()
}


fn part_b(inputs: &[i32]) -> i32 {
    unimplemented!()
}


#[derive(Debug)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug, Default, Eq, PartialEq)]
struct File {
    name: String,
    is_directory: bool,
    size: usize,
}
