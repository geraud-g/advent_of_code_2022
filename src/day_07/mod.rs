use advent_of_code::utils::arena_tree::ArenaTree;
use advent_of_code::utils::inputs::get_file;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE_CD: Regex = Regex::new(r"(?m)^\s*cd\s+(.+)\s*").unwrap();
    static ref RE_DIR: Regex = Regex::new(r"(?m)^dir (\w+)").unwrap();
    static ref RE_FILE: Regex = Regex::new(r"(?m)^(\d+)\s+(\w+)").unwrap();
}

pub fn day_07() {
    let commands = get_input();

    let solution_1 = part_one(&commands);
    println!("\t- Solution 1 is : {:?}", solution_1);

    let solution_2 = part_two(&commands);
    println!("\t- Solution 2 is : {:?}", solution_2);
}

fn get_input() -> ArenaTree<File> {
    let mut arena_tree = ArenaTree::default();
    let root_file = File::new_directory("/".to_string());
    let mut current_path_index = arena_tree.insert_node(root_file, None);
    let root_index = current_path_index;

    for chunk in get_file("./src/day_07/input.txt").split('$') {
        let mut lines = chunk.lines();
        if let Some(raw_command) = lines.next() {
            // Cd command
            if let Command::Cd(path) = get_command(raw_command) {
                current_path_index = match path.as_ref() {
                    ".." => arena_tree.get_unwrapped(current_path_index).parent.unwrap(),
                    "/" => root_index,
                    _ => {
                        if let Some(directory_idx) =
                            get_directory_idx(&arena_tree, current_path_index, path.as_ref())
                        {
                            directory_idx
                        } else {
                            arena_tree
                                .insert_node(File::new_directory(path), Some(current_path_index))
                        }
                    }
                }
            } else {
                // Ls command
                for line in lines {
                    if let Some(value) = RE_DIR.captures(line) {
                        let new_file =
                            File::new_directory(value.get(1).unwrap().as_str().to_string());
                        arena_tree.insert_node(new_file, Some(current_path_index));
                    } else if let Some(value) = RE_FILE.captures(line) {
                        let new_file = File::new_file(
                            value.get(2).unwrap().as_str().to_string(),
                            value.get(1).unwrap().as_str().parse().unwrap(),
                        );
                        arena_tree.insert_node(new_file, Some(current_path_index));
                    }
                }
            }
        }
    }
    arena_tree
}

fn get_command(line: &str) -> Command {
    if let Some(value) = RE_CD.captures(line) {
        Command::Cd(value.get(1).unwrap().as_str().to_string())
    } else {
        Command::Ls
    }
}

fn part_one(arena: &ArenaTree<File>) -> usize {
    let mut dir_list = vec![];
    get_directories_with_size_at_most(arena, 100_000, 0, &mut dir_list);
    dir_list.iter().sum::<usize>()
}

fn get_directories_with_size_at_most(
    arena: &ArenaTree<File>,
    max_size: usize,
    idx: usize,
    dir_list: &mut Vec<usize>,
) -> usize {
    let current_node = arena.get_unwrapped(idx);
    if !current_node.val.is_directory {
        return current_node.val.size;
    }

    let mut total = 0;
    for child in &current_node.children {
        total += get_directories_with_size_at_most(arena, max_size, *child, dir_list);
    }
    if total <= max_size {
        dir_list.push(total);
    }
    total
}

fn part_two(arena: &ArenaTree<File>) -> usize {
    let mut dir_list = vec![];
    let used_space = get_size(arena, 0);
    let free_space = 70000000 - used_space;
    let required_delete_space = 30000000 - free_space;

    get_files_sizes(arena, 0, &mut dir_list);
    let p = dir_list
        .iter()
        .filter(|&d| *d >= required_delete_space)
        .min()
        .unwrap();
    *p
}

fn get_files_sizes(arena: &ArenaTree<File>, idx: usize, dir_list: &mut Vec<usize>) -> usize {
    let current_node = arena.get_unwrapped(idx);
    if !current_node.val.is_directory {
        return current_node.val.size;
    }

    let mut total = 0;
    for child in &current_node.children {
        let child_node = arena.get_unwrapped(*child);
        total += get_files_sizes(arena, child_node.idx, dir_list);
    }
    dir_list.push(total);
    total
}

fn get_size(arena: &ArenaTree<File>, current_idx: usize) -> usize {
    arena
        .get_unwrapped(current_idx)
        .children
        .iter()
        .map(|child_idx| get_size(arena, *child_idx))
        .sum::<usize>()
        + arena.get_unwrapped(current_idx).val.size
}

fn get_directory_idx(arena: &ArenaTree<File>, current_idx: usize, path: &str) -> Option<usize> {
    for child_idx in &arena.get_unwrapped(current_idx).children {
        if arena.get_unwrapped(*child_idx).val.name.eq(path) {
            return Some(*child_idx);
        }
    }
    None
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

impl File {
    fn new_directory(name: String) -> Self {
        Self {
            name,
            is_directory: true,
            size: 0,
        }
    }

    fn new_file(name: String, size: usize) -> Self {
        Self {
            name,
            is_directory: false,
            size,
        }
    }
}
