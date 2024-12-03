#[path = "day1/day1.rs"] mod day1;
#[path = "day2/day2.rs"] mod day2;

fn main() {
    // You can now use functions or items from day1 module
    println!("Answer to day1 part1: {}", day1::solve_part1());
    println!("Answer to day1 part2: {}", day1::solve_part2());
    println!("Answer to day2 part1: {}", day2::solve_part1());
    println!("Answer to day2 part2: {}", day2::solve_part2());
}