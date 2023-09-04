use advent_of_code::utils::inputs::{get_file, LINE_ENDING};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

pub fn day_13() {
    let packet_data_pairs = get_input();

    let solution_1 = part_one(&packet_data_pairs);
    println!("\t- Solution 1 is : {}", solution_1);

    let mut packet_data = get_input_part_2();
    let solution_2 = part_two(&mut packet_data);
    println!("\t- Solution 2 is : {}", solution_2);
}

#[derive(Eq, PartialEq)]
pub struct PacketData {
    content: Vec<PacketData>,
    pub value: Option<u8>,
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        // If both values are integers, the lower integer should come first. If the left integer is
        // lower than the right integer, the inputs are in the right order. If the left integer is
        // higher than the right integer, the inputs are not in the right order. Otherwise, the inputs
        // are the same integer; continue checking the next part of the input.
        if let (Some(left_val), Some(right_val)) = (self.value, other.value) {
            return match left_val.cmp(&right_val) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => Ordering::Equal,
            };
        }

        // If both values are lists, compare the first value of each list, then the second value,
        // and so on. If the left list runs out of items first, the inputs are in the right order.
        // If the right list runs out of items first, the inputs are not in the right order. If the
        // lists are the same length and no comparison makes a decision about the order, continue
        // checking the next part of the input.
        let left_packet_len = self.content.len();
        let right_packet_len = other.content.len();
        if self.is_list() && other.is_list() {
            for i in 0..left_packet_len {
                if i >= left_packet_len || i >= right_packet_len {
                    break;
                }
                let val = self.content[i].cmp(&other.content[i]);
                if !val.is_eq() {
                    return val;
                }
            }

            if left_packet_len != right_packet_len {
                return left_packet_len.cmp(&right_packet_len);
            }
        }

        // If exactly one value is an integer, convert the integer to a list which contains that
        // integer as its only value, then retry the comparison. For example, if comparing [0,0,0] and 2
        // , convert the right value to [2] (a list containing 2); the result is then found by instead
        // comparing [0,0,0] and [2].
        if self.is_integer() {
            return PacketData {
                content: vec![PacketData {
                    content: vec![],
                    value: self.value,
                }],
                value: None,
            }
            .cmp(other);
        }

        if other.is_integer() {
            return self.cmp(&PacketData {
                content: vec![PacketData {
                    content: vec![],
                    value: other.value,
                }],
                value: None,
            });
        }
        Ordering::Equal
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Debug for PacketData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.value {
            write!(f, "{}", value)
        } else {
            write!(f, "{:?}", self.content)
        }
    }
}

impl FromStr for PacketData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = 0;
        let end = s.len();
        if let Some(packet) = get_packet(&s[start..end]) {
            Ok(packet)
        } else {
            panic!("Should not be empty")
        }
    }
}

impl PacketData {
    fn is_integer(&self) -> bool {
        self.value.is_some()
    }

    fn is_list(&self) -> bool {
        self.value.is_none()
    }
}

fn get_packet(s: &str) -> Option<PacketData> {
    let mut string = s;
    if !s.is_empty() && s.starts_with('[') && s.ends_with(']') {
        string = &s[1..s.len() - 1];
    }
    if string.is_empty() {
        return if s.is_empty() {
            None
        } else {
            Some(PacketData {
                content: vec![],
                value: None,
            })
        };
    }
    let mut bracket_depth = 0;
    let mut vec_packet_data = vec![];
    let mut start_value_idx = 0;
    if !string.is_empty() && string.chars().all(|c| c.is_numeric()) {
        return Some(PacketData {
            content: vec![],
            value: Some(string.parse::<u8>().unwrap()),
        });
    }
    for (idx, val) in string.chars().enumerate() {
        if val == ',' && bracket_depth == 0 {
            if let Some(new_packet_data) = get_packet(&string[start_value_idx..idx]) {
                vec_packet_data.push(new_packet_data);
            }
            start_value_idx = idx + 1;
        }
        if val == '[' {
            bracket_depth += 1;
        }
        if val == ']' {
            bracket_depth -= 1;
        }
    }

    if let Some(new_packet_data) = get_packet(&string[start_value_idx..]) {
        vec_packet_data.push(new_packet_data);
    }
    Some(PacketData {
        content: vec_packet_data,
        value: None,
    })
}

fn get_input() -> Vec<(PacketData, PacketData)> {
    let file = get_file("./src/day_13/input.txt");
    let split_separator = format!("{}{}", LINE_ENDING, LINE_ENDING);
    let mut packet_pairs = vec![];
    for chunk in file.split(&split_separator) {
        let values: Vec<&str> = chunk.split_whitespace().collect();
        let packet_left = PacketData::from_str(values[0]).unwrap();
        let packet_right = PacketData::from_str(values[1]).unwrap();
        packet_pairs.push((packet_left, packet_right));
    }
    packet_pairs
}

fn get_input_part_2() -> Vec<PacketData> {
    let file = get_file("./src/day_13/input.txt");
    let split_separator = format!("{}{}", LINE_ENDING, LINE_ENDING);
    let mut packet_pairs = vec![];
    for chunk in file.split(&split_separator) {
        let values: Vec<&str> = chunk.split_whitespace().collect();
        let packet_left = PacketData::from_str(values[0]).unwrap();
        let packet_right = PacketData::from_str(values[1]).unwrap();
        packet_pairs.push(packet_left);
        packet_pairs.push(packet_right);
    }
    packet_pairs.push(PacketData::from_str("[[2]]").unwrap());
    packet_pairs.push(PacketData::from_str("[[6]]").unwrap());
    packet_pairs
}

fn part_one(packet_data_pairs: &[(PacketData, PacketData)]) -> usize {
    let mut result = 0;
    for (idx, (left, right)) in packet_data_pairs.iter().enumerate() {
        if left.cmp(right).is_le() {
            result += idx + 1
        }
    }
    result
}

fn part_two(packet_data: &mut [PacketData]) -> usize {
    packet_data.sort();
    let divider_1_index = packet_data
        .iter()
        .position(|p| *p == PacketData::from_str("[[2]]").unwrap())
        .unwrap()
        + 1;
    let divider_2_index = packet_data
        .iter()
        .position(|p| *p == PacketData::from_str("[[6]]").unwrap())
        .unwrap()
        + 1;
    divider_1_index * divider_2_index
}
