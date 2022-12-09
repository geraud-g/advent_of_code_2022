use std::collections::HashSet;
use advent_of_code::utils::inputs::get_file;


pub fn day_09() {
    let motions = get_input();

    let solution_a = get_rope_tail_nodes_visit_nbr(&motions, 2);
    println!("\t- Solution A is : {}", solution_a);

    let solution_b = get_rope_tail_nodes_visit_nbr(&motions, 10);
    println!("\t- Solution B is : {}", solution_b);
}


fn get_input() -> Vec<Motion> {
    let mut instructions = vec![];
    for line in get_file("./src/day_09/input.txt").lines() {
        let values = line.split(' ').collect::<Vec<&str>>();
        let direction = match values[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Cannot get direction from value {}", values[0])
        };
        let move_nbr = values[1].parse().unwrap();
        instructions.push((direction, move_nbr))
    }
    instructions
}


fn get_rope_tail_nodes_visit_nbr(motions: &[Motion], rope_size: usize) -> usize {
    let mut rope = vec![(0, 0); rope_size];
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();

    for motion in motions {
        for _ in 0..motion.1 {
            rope[0] = move_head(rope[0], motion);
            for node_idx in 1..rope_size {
                rope[node_idx] = move_tail(rope[node_idx - 1], rope[node_idx]);
            }
            tail_positions.insert(rope[rope_size - 1]);
        }
    }
    tail_positions.len()
}

fn move_head(head: (i32, i32), motion: &Motion) -> (i32, i32) {
    let modifier = match motion.0 {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };
    let mut new_position = head;

    new_position = (
        new_position.0 + modifier.0,
        new_position.1 + modifier.1,
    );

    new_position
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    if points_touching(head, tail) {
        return tail;
    }
    let mut new_tail = tail;

    if tail.0 < head.0 {
        new_tail.0 += 1;
    }
    if tail.0 > head.0 {
        new_tail.0 -= 1;
    }
    if tail.1 < head.1 {
        new_tail.1 += 1;
    }
    if tail.1 > head.1 {
        new_tail.1 -= 1;
    }
    new_tail
}


fn points_touching(p1: (i32, i32), p2: (i32, i32)) -> bool {
    if p1 == p2 {
        return true;
    }
    let (x1, y1) = p1;
    let (x2, y2) = p2;
    (x1 == x2 - 1 || x1 == x2 + 1 || y1 == y2 - 1 || y1 == y2 + 1)
        && (x1 == x2 - 1 || x1 == x2 + 1 || x1 == x2)
        && (y1 == y2 || y1 == y2 - 1 || y1 == y2 + 1)
}


#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Motion = (Direction, usize);