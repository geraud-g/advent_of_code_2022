use advent_of_code::utils::inputs::get_file;


pub fn day_02() {
    let strategy_guide = get_input();

    let solution_a = part_a(&strategy_guide);
    println!("\t- Solution A is : {}", solution_a);

    let solution_b = part_b(&strategy_guide);
    println!("\t- Solution B is : {}", solution_b);
}


fn get_input() -> Vec<(Move, String)> {
    get_file("./src/day_02/input.txt")
        .lines()
        .map(
            |line| {
                let mut moves = line.trim().split(' ');
                (Move::from_str(moves.next().unwrap()), moves.next().unwrap().to_string())
            })
        .collect()
}


fn part_a(strategy_guide: &[(Move, String)]) -> u16 {
    let mut my_score = 0;

    for (op_move, value) in strategy_guide {
        let my_move = Move::from_str(value);
        my_score += my_move.get_score();
        my_score += my_move.get_result(op_move).get_result_value();
    }
    my_score
}


fn part_b(strategy_guide: &[(Move, String)]) -> u16 {
    let mut my_score = 0;
    for (op_move, value) in strategy_guide {
        let expected_result = match value.as_ref() {
            "X" => Result::Loss,
            "Y" => Result::Draw,
            _ => Result::Win
        };
        let new_move = op_move.get_move_from_expected_result(&expected_result);
        my_score += new_move.get_score();
        my_score += expected_result.get_result_value();
    }
    my_score
}


#[derive(Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_str(value: &str) -> Self {
        match value {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Cannot get Move from str `{}`", value)
        }
    }

    fn get_score(&self) -> u16 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn get_result(&self, other: &Self) -> Result {
        match (self, other) {
            (Move::Rock, Move::Paper) => Result::Loss,
            (Move::Rock, Move::Scissors) => Result::Win,
            (Move::Paper, Move::Scissors) => Result::Loss,
            (Move::Paper, Move::Rock) => Result::Win,
            (Move::Scissors, Move::Rock) => Result::Loss,
            (Move::Scissors, Move::Paper) => Result::Win,
            _ => Result::Draw
        }
    }

    fn get_move_from_expected_result(&self, result: &Result) -> Self {
        match self {
            Move::Rock => {
                match result {
                    Result::Win => Move::Paper,
                    Result::Loss => Move::Scissors,
                    _ => Move::Rock
                }
            }
            Move::Paper => {
                match result {
                    Result::Win => Move::Scissors,
                    Result::Loss => Move::Rock,
                    _ => Move::Paper
                }
            }
            Move::Scissors => {
                match result {
                    Result::Win => Move::Rock,
                    Result::Loss => Move::Paper,
                    _ => Move::Scissors
                }
            }
        }
    }
}

#[derive(Debug)]
enum Result {
    Win,
    Draw,
    Loss,
}

impl Result {
    fn get_result_value(&self) -> u16 {
        match self {
            Result::Loss => 0,
            Result::Draw => 3,
            Result::Win => 6
        }
    }
}