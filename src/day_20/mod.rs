use advent_of_code::utils::inputs::get_file;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
struct Number {
    id: usize,
    value: i64,
}

pub fn day_20() {
    let file = get_input("./src/day_20/input.txt");

    let solution_1 = part_one(file.clone());
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(file);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn get_input(file: &str) -> VecDeque<Number> {
    get_file(file)
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .enumerate()
        .map(|(id, value)| Number { id, value })
        .collect()
}

fn part_one(mut file: VecDeque<Number>) -> i64 {
    for number_id in 0..file.len() {
        mix_file(&mut file, number_id);
    }
    [1000, 2000, 3000]
        .iter()
        .map(|val| find_nth_number_after_zero(&file, *val))
        .sum()
}

fn part_two(mut file: VecDeque<Number>) -> i64 {
    file = file
        .iter()
        .map(|n| Number {
            value: n.value * 811589153,
            ..*n
        })
        .collect();

    for _ in 0..10 {
        for number_id in 0..file.len() {
            mix_file(&mut file, number_id);
        }
    }
    [1000, 2000, 3000]
        .iter()
        .map(|val| find_nth_number_after_zero(&file, *val))
        .sum()
}

fn mix_file(file: &mut VecDeque<Number>, number_id: usize) {
    let number_index = get_number_index_by_id(file, number_id);
    let number = file[number_index];
    if number.value == 0 {
        return;
    }

    file.rotate_left(number_index);
    file.pop_front();
    if number.value < 0 {
        file.rotate_right((number.value.unsigned_abs() as usize) % file.len());
    } else {
        file.rotate_left((number.value as usize) % file.len())
    }
    file.insert(0, number);
}

fn find_nth_number_after_zero(file: &VecDeque<Number>, n: usize) -> i64 {
    let zero_idx = file.iter().position(|&r| r.value == 0).unwrap();
    let nth_idx = (zero_idx + n) % file.len();
    file[nth_idx].value
}

fn get_number_index_by_id(file: &VecDeque<Number>, number_id: usize) -> usize {
    file.iter().position(|&r| r.id == number_id).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_nth_number_after_zero() {
        let file = get_input("./src/day_20/input_example.txt");
        assert_eq!(find_nth_number_after_zero(&file, 1000), -2);
        assert_eq!(find_nth_number_after_zero(&file, 2000), 3);
        assert_eq!(find_nth_number_after_zero(&file, 3000), -3);
    }
}
