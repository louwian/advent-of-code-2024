use std::env;

#[path = "day1/day1.rs"] mod day1;
#[path = "day2/day2.rs"] mod day2;
#[path = "day3/day3.rs"] mod day3;
#[path = "day4/day4.rs"] mod day4;

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
            // println!("Answer to day4 part2: {}", day4::solve_part2());
        }
        _ => {
            eprintln!("Unknown day: {}", day);
            std::process::exit(1);
        }
    }
}