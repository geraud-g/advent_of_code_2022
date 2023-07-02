use advent_of_code::utils::inputs::get_file;
use std::collections::HashSet;
use std::str::FromStr;

pub fn day_10() {
    let op_list = get_input();

    let solution_1 = part_one(&op_list);
    println!("\t- Solution 1 is : {}", solution_1);

    print!("\t- Solution 2 is :");
    part_two(&op_list);
}

fn get_input() -> Vec<Op> {
    get_file("./src/day_10/input.txt")
        .lines()
        .map(|line| line.parse::<Op>().unwrap())
        .collect()
}

fn part_one(op_list: &[Op]) -> i32 {
    let mut register_x = 1;
    let mut current_cycle = 0;
    let signal_steps: HashSet<i32> = vec![20, 60, 100, 140, 180, 220].into_iter().collect();
    let mut sum_signal_strength = 0;

    for op in op_list {
        current_cycle += 1;
        if signal_steps.contains(&current_cycle) {
            sum_signal_strength += current_cycle * register_x;
        };
        if let Op::AddX(val) = op {
            current_cycle += 1;
            if signal_steps.contains(&current_cycle) {
                sum_signal_strength += current_cycle * register_x;
            };
            register_x += val;
        }
    }
    sum_signal_strength
}

fn part_two(op_list: &[Op]) {
    let mut register_x = 1;
    let mut current_cycle = 0;

    draw_pixel(register_x, current_cycle);
    for op in op_list.iter() {
        current_cycle += 1;
        draw_pixel(register_x, current_cycle);

        if let Op::AddX(val) = op {
            current_cycle += 1;
            register_x += val;
            draw_pixel(register_x, current_cycle);
        }
    }
}

fn draw_pixel(sprite_position: i32, current_cycle: i32) {
    let current_cycle = current_cycle % 40;
    if current_cycle == 0 {
        println!()
    }
    if current_cycle >= sprite_position - 1 && current_cycle <= sprite_position + 1 {
        print!("█")
    } else {
        print!(" ")
    }
}

#[derive(Debug)]
enum Op {
    AddX(i32),
    Noop,
}

impl FromStr for Op {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let op = words.next().unwrap();

        match op {
            "addx" => {
                let x = words.next().unwrap().parse::<i32>().unwrap();
                Ok(Op::AddX(x))
            }
            "noop" => Ok(Op::Noop),
            _ => Err(()),
        }
    }
}
