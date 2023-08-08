use advent_of_code::utils::inputs::get_file;
use std::collections::HashMap;

const MY_KEY: &str = "humn";

pub fn day_21() {
    let node = get_input();

    let solution_1 = part_one(&node);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(&node);
    println!("\t- Solution 2 is : {}", solution_2);
}

fn get_input() -> Node {
    let mut node = HashMap::new();
    let file = get_file("./src/day_21/input.txt");
    for line in file.lines() {
        let split = line.split(": ").collect::<Vec<_>>();
        let name = split[0].to_string();
        let split = split[1].split(' ').collect::<Vec<_>>();
        node.insert(name, split);
    }
    build_node(&node, "root")
}

fn build_node(monkeys: &HashMap<String, Vec<&str>>, name: &str) -> Node {
    let monkey = monkeys.get(name).unwrap();
    let expression = match monkey[..] {
        [_, "+", _] => Expression::Add,
        [_, "-", _] => Expression::Sub,
        [_, "*", _] => Expression::Mul,
        [_, "/", _] => Expression::Div,
        [value] => Expression::Value(value.parse().unwrap()),
        _ => panic!("Cannot parse {:?}", monkey),
    };
    let (left, right) = if let Expression::Value(_) = expression {
        (None, None)
    } else {
        (
            Some(Box::new(build_node(monkeys, monkey[0]))),
            Some(Box::new(build_node(monkeys, monkey[2]))),
        )
    };

    Node {
        name: name.to_string(),
        left,
        right,
        expression,
    }
}

#[derive(Debug)]
enum Expression {
    Add,
    Sub,
    Mul,
    Div,
    Value(i64),
}

#[derive(Debug)]
struct Node {
    name: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    expression: Expression,
}

impl Node {
    fn evaluate(&self) -> i64 {
        match self.expression {
            Expression::Add => self.get_left().evaluate() + self.get_right().evaluate(),
            Expression::Sub => self.get_left().evaluate() - self.get_right().evaluate(),
            Expression::Mul => self.get_left().evaluate() * self.get_right().evaluate(),
            Expression::Div => self.get_left().evaluate() / self.get_right().evaluate(),
            Expression::Value(val) => val,
        }
    }

    fn get_left(&self) -> &Node {
        self.left.as_ref().unwrap()
    }

    fn get_right(&self) -> &Node {
        self.right.as_ref().unwrap()
    }

    fn contains_name(&self, name: &str) -> bool {
        if self.name == name {
            true
        } else if !matches!(self.expression, Expression::Value(_)) {
            self.get_left().contains_name(name) || self.get_right().contains_name(name)
        } else {
            false
        }
    }

    fn find_x(&self, mut x_value: i64) -> i64 {
        if self.name == MY_KEY {
            return x_value;
        }
        let left = self.get_left();
        let right = self.get_right();
        if left.contains_name(MY_KEY) {
            x_value = simplify_value(x_value, &self.expression, right.evaluate(), false);
            left.find_x(x_value)
        } else {
            x_value = simplify_value(x_value, &self.expression, left.evaluate(), true);
            right.find_x(x_value)
        }
    }
}

fn part_one(nodes: &Node) -> i64 {
    nodes.evaluate()
}

fn part_two(nodes: &Node) -> i64 {
    let left = nodes.get_left();
    let right = nodes.get_right();
    if left.contains_name(MY_KEY) {
        left.find_x(right.evaluate())
    } else {
        right.find_x(left.evaluate())
    }
}

fn simplify_value(
    x_value: i64,
    operand: &Expression,
    other_value: i64,
    x_is_right_side: bool,
) -> i64 {
    match operand {
        Expression::Add => x_value - other_value,
        Expression::Sub => {
            if x_is_right_side {
                -x_value + other_value
            } else {
                x_value + other_value
            }
        }
        Expression::Mul => x_value / other_value,
        Expression::Div => {
            if x_is_right_side {
                other_value / x_value
            } else {
                x_value * other_value
            }
        }
        _ => panic!("Cannot process {:?}", operand),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_solution() {
        // 150 = x - 3
        assert_eq!(153, simplify_value(150, &Expression::Sub, 3, false));
        // 150 = 3 - x
        assert_eq!(-147, simplify_value(150, &Expression::Sub, 3, true));

        // 150 = 3 + x
        assert_eq!(147, simplify_value(150, &Expression::Add, 3, false));
        // 150 = x + 3
        assert_eq!(147, simplify_value(150, &Expression::Add, 3, true));

        // 150 = x * 3
        assert_eq!(50, simplify_value(150, &Expression::Mul, 3, false));
        // 150 = 3 * x
        assert_eq!(50, simplify_value(150, &Expression::Mul, 3, true));

        // 150 = x / 3
        assert_eq!(450, simplify_value(150, &Expression::Div, 3, false));
        // 150 = 3 / x
        assert_eq!(1 / 50, simplify_value(150, &Expression::Div, 3, true));
    }
}
