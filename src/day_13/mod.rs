use advent_of_code::utils::inputs::{get_file, LINE_ENDING};
use std::fmt;
use std::str::FromStr;

pub fn day_13() {
    let packet_data_pairs = get_input();

    let solution_1 = part_one(&packet_data_pairs);
    println!("\t- Solution 1 is : {}", solution_1);
    println!("Should be 5506")

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
            write!(f, "[{}]", value)
        } else if self.value.is_none() && self.content.is_empty() {
            write!(f, "[]")
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
            panic!("Should not be empty ?")
            // Ok(PacketData { content: vec![], value: PacketDataValue::Empty })
        }
    }
}

pub struct PacketDataIterator<'a> {
    packet: &'a PacketData,
    index: usize,
}

impl<'a> PacketDataIterator<'a> {
    fn new(packet: &'a PacketData) -> Self {
        PacketDataIterator { packet, index: 0 }
    }
}

impl<'a> Iterator for PacketDataIterator<'a> {
    type Item = &'a PacketData;

    fn next(&mut self) -> Option<Self::Item> {
        let mut index = 0;
        let mut stack = vec![self.packet];

        while let Some(packet) = stack.pop() {
            if index == self.index {
                self.index += 1;
                return Some(packet);
            }
            index += 1;
            for child in packet.content.iter().rev() {
                stack.push(child);
            }
        }
        None
    }
}

impl PacketData {
    fn iter(&self) -> PacketDataIterator {
        PacketDataIterator::new(self)
    }
}

fn get_packet(s: &str) -> Option<PacketData> {
    let mut string = s.clone();
    if s.len() > 0 && s.starts_with('[') && s.ends_with(']') {
        string = &s[1..s.len() - 1];
    }
    if string.len() == 0 {
        return if s.len() == 0 {
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
    if string.len() > 0 && string.chars().all(|c| c.is_numeric()) {
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

impl Iterator for PacketData {
    type Item = Option<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
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
        println!("########## Eval:\n {:?}\n {:?}", left, right);
        let mut left_packets = left.iter();
        let mut right_packets = right.iter();
        let a = left_packets.next();
        let b = right_packets.next();
        if is_in_right_order(a, b, &mut left_packets, &mut right_packets) {
            println!("-> RIGHT order");
            result += idx + 1
        } else {
            println!("-> WRONG order");
        }
    }
    result
}

fn is_in_right_order(
    left_packet_o: Option<&PacketData>,
    right_packet_o: Option<&PacketData>,
    left: &mut PacketDataIterator,
    right: &mut PacketDataIterator,
) -> bool {
    match (left_packet_o, right_packet_o) {
        (Some(left_packet), Some(right_packet)) => {
            // Both integer:
            // If both values are integers, the lower integer should come first. If the left integer
            // is lower than the right integer, the inputs are in the right order. If the left
            // integer is higher than the right integer, the inputs are not in the right order.
            // Otherwise, the inputs are the same integer; continue checking the next part of the
            // input.
            if let (Some(left_value), Some(right_value)) = (left_packet.value, right_packet.value) {
                if left_value != right_value {
                    return left_value < right_value;
                }
                return is_in_right_order(left.next(), right.next(), left, right);
            }

            // If exactly one value is an integer, convert the integer to a list which contains
            // that integer as its only value, then retry the comparison. For example, if
            // comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2);
            // the result is then found by instead comparing [0,0,0] and [2].
            if left_packet.value.is_some() {
                return if right_packet.content.is_empty() {
                    false
                } else {
                    is_in_right_order(left_packet_o, right.next(), left, right)
                };
            }
            if right_packet.value.is_some() {
                return if left_packet.content.is_empty() {
                    true
                } else {
                    is_in_right_order(left.next(), right_packet_o, left, right)
                };
            }

            // If both values are lists, compare the first value of each list, then the second
            // value, and so on. If the left list runs out of items first, the inputs are in the
            // right order. If the right list runs out of items first, the inputs are not in the
            // right order. If the lists are the same length and no comparison makes a decision
            // about the order, continue checking the next part of the input.
            if left_packet.value.is_none()
                && !left_packet.content.is_empty()
                && right_packet.value.is_none()
                && !right_packet.content.is_empty()
            {
                return is_in_right_order(left.next(), right.next(), left, right);
            }
            if left_packet.content.is_empty() && !right_packet.content.is_empty() {
                return true;
            }
            if !left_packet.content.is_empty() && right_packet.content.is_empty() {
                return false;
            }
            // TODO : Empty vs Empty, Empty vs None, None vs None
            is_in_right_order(left.next(), right.next(), left, right);
        }
        (None, None) => panic!("None - None is not handled"),
        (None, _) => return true,
        (_, None) => return false,
    }

    false
}
