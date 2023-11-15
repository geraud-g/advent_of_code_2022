use advent_of_code::utils::inputs::get_file;

pub fn day_25() {
    let numbers = get_input();

    let solution_1 = part_one(&numbers);
    println!("\t- Solution 1 is : {}", solution_1);
}

fn get_input() -> Vec<i64> {
    get_file("./src/day_25/input.txt")
        .lines()
        .map(snafu_to_decimal)
        .collect()
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    snafu
        .chars()
        .rev()
        .enumerate()
        .map(|(idx, digit)| {
            let value: i64 = match digit {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!("Cannot convert SNAFU value {} to i64", digit),
            };
            value * 5_i64.pow(idx as u32)
        })
        .sum()
}

fn part_one(numbers: &[i64]) -> String {
    decimal_to_snafu(numbers.iter().sum())
}

fn decimal_to_snafu(mut decimal: i64) -> String {
    let mut snafu = String::new();

    while decimal > 0 {
        let snafu_digit = match decimal % 5 {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            _ => unreachable!(),
        };
        decimal = (decimal as f64 / 5.).round() as i64;
        snafu.insert(0, snafu_digit)
    }
    snafu
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal_to_snafu() {
        let values = vec![
            (3, "1="),
            (7, "12"),
            (11, "21"),
            (31, "111"),
            (32, "112"),
            (37, "122"),
            (107, "1-12"),
            (198, "2=0="),
            (201, "2=01"),
            (353, "1=-1="),
            (906, "12111"),
            (1257, "20012"),
            (1747, "1=-0-2"),
            (4890, "2=-1=0"),
            (12345, "1-0---0"),
            (314159265, "1121-1110-1=0"),
        ];
        for (value, expected) in values {
            assert_eq!(&decimal_to_snafu(value), expected)
        }
    }
}
