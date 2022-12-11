use advent_of_code::utils::inputs::{get_file, LINE_ENDING};
use std::str::FromStr;
use itertools::Itertools;


pub fn day_11() {
    let solution_a = play_rounds(&mut get_input(), 20, true);
    println!("\t- Solution A is : {}", solution_a);

    let solution_b = play_rounds(&mut get_input(), 10000, false);
    println!("\t- Solution B is : {}", solution_b);
}


fn get_input() -> Vec<Monkey> {
    let split_separator = format!("{}{}", LINE_ENDING, LINE_ENDING);
    get_file("./src/day_11/input.txt")
        .split(&split_separator)
        .map(|line| Monkey::from_str(line).unwrap())
        .collect()
}


fn play_rounds(monkeys: &mut [Monkey], round_nbr: usize, puzzle_part_one: bool) -> u128 {
    for _ in 0..round_nbr {
        play_round(monkeys, puzzle_part_one)
    }
    monkeys
        .iter()
        .map(|m| m.monkey_business)
        .sorted()
        .rev()
        .take(2)
        .product()
}


fn play_round(monkeys: &mut [Monkey], puzzle_part_one: bool) {
    let mut new_item_values_and_owners = vec![];
    let common_divisor: u128 = monkeys.iter().map(|m| m.test_divisible_by).product();

    for monkey_idx in 0..monkeys.len() {
        for item_idx in 0..monkeys[monkey_idx].items.len() {
            monkeys[monkey_idx].monkey_business += 1;
            let item = monkeys[monkey_idx].items[item_idx];
            let mut new_item_value = apply_operation(&monkeys[monkey_idx].operation, &item);
            if puzzle_part_one {
                new_item_value /= 3;
            } else {
                new_item_value %= common_divisor;
            }

            if new_item_value % monkeys[monkey_idx].test_divisible_by == 0 {
                new_item_values_and_owners.push((new_item_value, monkeys[monkey_idx].if_true));
            } else {
                new_item_values_and_owners.push((new_item_value, monkeys[monkey_idx].if_false));
            }
        }
        for (item_value, owner_idx) in &new_item_values_and_owners {
            monkeys[*owner_idx].items.push(*item_value);
        }
        monkeys[monkey_idx].items.clear();
        new_item_values_and_owners.clear();
    }
}


fn apply_operation(operation: &(Value, Op, Value), value: &u128) -> u128 {
    let left_value = match operation.0 {
        Value::RawValue(val) => val,
        Value::Old => *value
    };
    let right_value = match operation.2 {
        Value::RawValue(val) => val,
        Value::Old => *value
    };
    match operation.1 {
        Op::Add => (left_value + right_value),
        Op::Mult => (left_value * right_value)
    }
}


#[derive(Debug)]
enum Value {
    Old,
    RawValue(u128),
}

impl FromStr for Value {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "old" => Ok(Self::Old),
            _ => Ok(Self::RawValue(s.parse().unwrap()))
        }
    }
}


#[derive(Debug)]
enum Op {
    Add,
    Mult,
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "*" => Ok(Self::Mult),
            "+" => Ok(Self::Add),
            _ => Err("Wrong value".to_owned())
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<u128>,
    operation: (Value, Op, Value),
    test_divisible_by: u128,
    if_true: usize,
    if_false: usize,
    monkey_business: u128,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let header = lines.next().ok_or("missing Id")?;
        let items = lines.next().ok_or("missing items")?;
        let operation = lines.next().ok_or("missing operation")?;
        let test = lines.next().ok_or("missing test")?;
        let raw_if_true = lines.next().ok_or("missing if_true")?;
        let raw_if_false = lines.next().ok_or("missing if_false")?;

        // ID
        let parts: Vec<&str> = header.split(':').collect();
        let id = parts[0].split_whitespace().last().ok_or("missing number")?
            .parse().expect("invalid number");

        // Starting items
        let parts: Vec<&str> = items.split(':').collect();
        let starting_items = parts[1]
            .split(',')
            .map(str::trim)
            .map(|s| s.parse::<u128>().unwrap())
            .collect();

        // Operations
        let parts: Vec<&str> = operation.split("= ").collect();
        let values: Vec<&str> = parts[1].split_whitespace().collect();
        let operation = (
            Value::from_str(values[0]).unwrap(),
            Op::from_str(values[1]).unwrap(),
            Value::from_str(values[2]).unwrap(),
        );

        // Divisible By
        let parts: Vec<&str> = test.split(" by ").collect();
        let test_divisible_by = parts[1].parse().unwrap();

        // Is true
        let parts: Vec<&str> = raw_if_true.split(" monkey ").collect();
        let if_true = parts[1].parse().unwrap();

        // Is false
        let parts: Vec<&str> = raw_if_false.split(" monkey ").collect();
        let if_false = parts[1].parse().unwrap();

        Ok(Monkey {
            id,
            items: starting_items,
            operation,
            test_divisible_by,
            if_true,
            if_false,
            monkey_business: 0,
        })
    }
}
