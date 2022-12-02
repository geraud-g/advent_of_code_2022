use std::env;

mod day_01;
mod day_02;
// mod day_03;
// mod day_04;
// mod day_05;
// mod day_06;
// mod day_07;
// mod day_08;
// mod day_09;
// mod day_10;
// mod day_11;
// mod day_12;
// mod day_13;
// mod day_14;
// mod day_15;
// mod day_16;
// mod day_17;
// mod day_18;
// mod day_19;
// mod day_20;
// mod day_21;
// mod day_22;
// mod day_23;
// mod day_24;
// mod day_25;


fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => solve_day(args[1].trim().parse().expect("You must enter a number between 1 and 25.")),
        _ => invalid_input()
    }
}


fn solve_day(day: i32) {
    if !(1..=25).contains(&day) {
        invalid_input();
    }
    let fn_day = match day {
        1 => day_01::day_01,
        2 => day_02::day_02,
        // 3 => day_03::day_03,
        // 4 => day_04::day_04,
        // 5 => day_05::day_05,
        // 6 => day_06::day_06,
        // 7 => day_07::day_07,
        // 8 => day_08::day_08,
        // 9 => day_09::day_09,
        // 10 => day_10::day_10,
        // 11 => day_11::day_11,
        // 12 => day_12::day_12,
        // 13 => day_13::day_13,
        // 14 => day_14::day_14,
        // 15 => day_15::day_15,
        // 16 => day_16::day_16,
        // 17 => day_17::day_17,
        // 18 => day_18::day_18,
        // 19 => day_19::day_19,
        // 20 => day_20::day_20,
        // 21 => day_21::day_21,
        // 22 => day_22::day_22,
        // 23 => day_23::day_23,
        // 24 => day_24::day_24,
        _ => unimplemented!(),
    };
    println!("# Processing Day {} :", day);
    fn_day()
}

fn invalid_input() {
    panic!("You must enter a number between 1 and 25.")
}