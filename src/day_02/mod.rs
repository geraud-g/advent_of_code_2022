use advent_of_code::utils::inputs::get_file;
use std::str::FromStr;

pub fn day_02() {
    let strategy_guide = get_input();

    let solution_1 = part_one(&strategy_guide);
    println!("\t- Solution 1 is : {}", solution_1);

    let solution_2 = part_two(&strategy_guide);
    println!("\t- Solution 2 is : {}", solution_2);
}

/// Returns a vector of tuple (GameAction, `String`), where `String` is how the round needs to end.
fn get_input() -> Vec<(GameAction, String)> {
    get_file("./src/day_02/input.txt")
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> (GameAction, String) {
    let mut moves = line.trim().split(' ');
    (
        GameAction::from_str(moves.next().unwrap()).unwrap(),
        moves.next().unwrap().to_string(),
    )
}

fn part_one(strategy_guide: &[(GameAction, String)]) -> u16 {
    let mut my_score = 0;

    for (op_move, value) in strategy_guide {
        let my_move = GameAction::from_str(value).expect("Cannot parse {value}");
        my_score += my_move.get_score();
        my_score += my_move.get_result(op_move).get_result_value();
    }
    my_score
}

fn part_two(strategy_guide: &[(GameAction, String)]) -> u16 {
    let mut my_score = 0;
    for (op_move, value) in strategy_guide {
        let expected_result = match value.as_ref() {
            "X" => GameResult::Loss,
            "Y" => GameResult::Draw,
            _ => GameResult::Win,
        };
        let new_move = op_move.get_action_from_expected_result(&expected_result);
        my_score += new_move.get_score();
        my_score += expected_result.get_result_value();
    }
    my_score
}

#[derive(Debug)]
enum GameAction {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for GameAction {
    type Err = String;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err("Wrong value `{value}`".to_owned()),
        }
    }
}

impl GameAction {
    fn get_score(&self) -> u16 {
        match self {
            GameAction::Rock => 1,
            GameAction::Paper => 2,
            GameAction::Scissors => 3,
        }
    }

    fn get_result(&self, other: &Self) -> GameResult {
        match (self, other) {
            (GameAction::Rock, GameAction::Paper) => GameResult::Loss,
            (GameAction::Rock, GameAction::Scissors) => GameResult::Win,
            (GameAction::Paper, GameAction::Scissors) => GameResult::Loss,
            (GameAction::Paper, GameAction::Rock) => GameResult::Win,
            (GameAction::Scissors, GameAction::Rock) => GameResult::Loss,
            (GameAction::Scissors, GameAction::Paper) => GameResult::Win,
            _ => GameResult::Draw,
        }
    }

    fn get_action_from_expected_result(&self, result: &GameResult) -> Self {
        match self {
            GameAction::Rock => match result {
                GameResult::Win => GameAction::Paper,
                GameResult::Loss => GameAction::Scissors,
                _ => GameAction::Rock,
            },
            GameAction::Paper => match result {
                GameResult::Win => GameAction::Scissors,
                GameResult::Loss => GameAction::Rock,
                _ => GameAction::Paper,
            },
            GameAction::Scissors => match result {
                GameResult::Win => GameAction::Rock,
                GameResult::Loss => GameAction::Paper,
                _ => GameAction::Scissors,
            },
        }
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

impl GameResult {
    fn get_result_value(&self) -> u16 {
        match self {
            GameResult::Loss => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        }
    }
}
