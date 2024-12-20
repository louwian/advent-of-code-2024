use std::env;

#[path = "day1/day1.rs"] mod day1;
#[path = "day2/day2.rs"] mod day2;
#[path = "day3/day3.rs"] mod day3;
#[path = "day4/day4.rs"] mod day4;
#[path = "day5/day5.rs"] mod day5;
#[path = "day6/day6.rs"] mod day6;
#[path = "day7/day7.rs"] mod day7;
#[path = "day8/day8.rs"] mod day8;
#[path = "day9/day9.rs"] mod day9;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <day>", args[0]);
        std::process::exit(1);
    }

    let day = &args[1];
    match day.as_str() {
        "day1" => {
            println!("Answer to day1 part1: {}", day1::solve_part1());
            println!("Answer to day1 part2: {}", day1::solve_part2());
        }
        "day2" => {
            println!("Answer to day2 part1: {}", day2::solve_part1());
            println!("Answer to day2 part2: {}", day2::solve_part2());
        }
        "day3" => {
            println!("Answer to day3 part1: {}", day3::solve_part1());
            println!("Answer to day3 part2: {}", day3::solve_part2());
        }
        "day4" => {
            println!("Answer to day4 part1: {}", day4::solve_part1());
            println!("Answer to day4 part2: {}", day4::solve_part2());
        }
        "day5" => {
            println!("Answer to day5 part1: {}", day5::solve_part1());
            println!("Answer to day5 part2: {}", day5::solve_part2());
        }
        "day6" => {
            println!("Answer to day6 part1: {}", day6::solve_part1());
            println!("Answer to day6 part2: {}", day6::solve_part2());
        }
        "day7" => {
            println!("Answer to day7 part1: {}", day7::solve_part1());
            println!("Answer to day7 part2: {}", day7::solve_part2());
        }
        "day8" => {
            println!("Answer to day8 part1: {}", day8::solve_part1());
            println!("Answer to day8 part2: {}", day8::solve_part2());
        }
        "day9" => {
            println!("Answer to day9 part1: {}", day9::solve_part1());
            println!("Answer to day9 part2: {}", day9::solve_part2());
        }
        _ => {
            eprintln!("Unknown day: {}", day);
            std::process::exit(1);
        }
    }
}