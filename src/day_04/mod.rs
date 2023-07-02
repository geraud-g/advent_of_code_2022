use advent_of_code::parse_input;
use advent_of_code::utils::inputs::get_file;
use itertools::Itertools;

type Section = (u16, u16);
type Pair = (Section, Section);

pub fn day_04() {
    let pairs = get_input();

    let solution_1 = part_one(&pairs);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(&pairs);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn get_input() -> Vec<Pair> {
    get_file("./src/day_04/input.txt")
        .lines()
        .map(line_to_pair)
        .collect()
}

fn line_to_pair(line: &str) -> Pair {
    line.split(',')
        .map(|s| {
            s.split('-')
                .map(|s2| parse_input!(s2, u16))
                .collect_tuple()
                .unwrap()
        })
        .collect_tuple()
        .unwrap()
}

fn part_one(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|p| contains(&p.0, &p.1) || contains(&p.1, &p.0))
        .count()
}

fn contains(section_a: &Section, section_b: &Section) -> bool {
    is_part_of(section_a.0, section_b) && is_part_of(section_a.1, section_b)
}

fn part_two(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|p| overlap(&p.0, &p.1) || overlap(&p.1, &p.0))
        .count()
}

fn overlap(section_a: &Section, section_b: &Section) -> bool {
    is_part_of(section_a.0, section_b) || is_part_of(section_a.1, section_b)
}

fn is_part_of(value: u16, section: &Section) -> bool {
    value >= section.0 && value <= section.1
}
