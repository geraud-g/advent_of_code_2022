use itertools::Itertools;
use advent_of_code::parse_input;
use advent_of_code::utils::inputs::get_file;

type Section = (u16, u16);
type Pair = (Section, Section);


pub fn day_04() {
    let pairs = get_input();

    let solution_a = part_a(&pairs);
    println!("\t- Solution A is : {}", solution_a);

    let solution_b = part_b(&pairs);
    println!("\t- Solution B is : {}", solution_b);
}


fn get_input() -> Vec<Pair> {
    get_file("./src/day_04/input.txt")
        .lines()
        .map(|line| line_to_pair(line))
        .collect()
}


fn line_to_pair(line: &str) -> Pair {
    line
        .split(',')
        .map(|s| s.split('-').map(|s2| parse_input!(s2, u16)).collect_tuple().unwrap())
        .collect_tuple()
        .unwrap()
}


fn part_a(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|p| contains(&p.0, &p.1) || contains(&p.1, &p.0))
        .count()
}


fn contains(section_a: &Section, section_b: &Section) -> bool {
    is_part_of(section_a.0, section_b) && is_part_of(section_a.1, section_b)
}


fn part_b(pairs: &[Pair]) -> usize {
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