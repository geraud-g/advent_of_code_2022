use advent_of_code::utils::inputs::{get_file, LINE_ENDING};
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

pub fn day_13() {
    let packet_data_pairs = get_input();

    let solution_1 = part_one(&packet_data_pairs);
    println!("\t- Solution 1 is : {}", solution_1);

    // let solution_2 = part_two(&inputs);
    // println!("\t- Solution 2 is : {}", solution_2);
}

pub struct PacketData {
    content: Vec<PacketData>,
    pub value: Option<u8>,
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

// TODO : Better split ?
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

fn part_one(packet_data_pairs: &[(PacketData, PacketData)]) -> usize {
    let mut result = 0;
    for (idx, (left, right)) in packet_data_pairs.iter().enumerate() {
        if let Some(true) = is_in_right_order(left, right) {
            result += idx + 1
        }
    }
    result
}

fn is_in_right_order(left_packet: &PacketData, right_packet: &PacketData) -> Option<bool> {
    // If both values are integers, the lower integer should come first. If the left integer is
    // lower than the right integer, the inputs are in the right order. If the left integer is
    // higher than the right integer, the inputs are not in the right order. Otherwise, the inputs
    // are the same integer; continue checking the next part of the input.
    if let (Some(left_val), Some(right_val)) = (left_packet.value, right_packet.value) {
        return match left_val.cmp(&right_val) {
            Ordering::Greater => Some(false),
            Ordering::Less => Some(true),
            Ordering::Equal => None,
        };
    }

    // If both values are lists, compare the first value of each list, then the second value,
    // and so on. If the left list runs out of items first, the inputs are in the right order.
    // If the right list runs out of items first, the inputs are not in the right order. If the
    // lists are the same length and no comparison makes a decision about the order, continue
    // checking the next part of the input.
    let left_packet_len = left_packet.content.len();
    let right_packet_len = right_packet.content.len();
    if left_packet.is_list() && right_packet.is_list() {
        for i in 0..left_packet_len {
            if i >= left_packet_len || i >= right_packet_len {
                break;
            }
            let val = is_in_right_order(&left_packet.content[i], &right_packet.content[i]);
            if val.is_some() {
                return val;
            }
        }

        if left_packet_len != right_packet_len {
            return Some(left_packet_len < right_packet_len);
        }
        // TODO compare les tailles direct
        // TODO puis chaque element un a un
    }

    // If exactly one value is an integer, convert the integer to a list which contains that
    // integer as its only value, then retry the comparison. For example, if comparing [0,0,0] and 2
    // , convert the right value to [2] (a list containing 2); the result is then found by instead
    // comparing [0,0,0] and [2].
    if left_packet.is_integer() {
        return is_in_right_order(
            &PacketData {
                content: vec![PacketData {
                    content: vec![],
                    value: left_packet.value,
                }],
                value: None,
            },
            right_packet,
        );
    }

    if right_packet.is_integer() {
        return is_in_right_order(
            left_packet,
            &PacketData {
                content: vec![PacketData {
                    content: vec![],
                    value: right_packet.value,
                }],
                value: None,
            },
        );
    }
    None
}
