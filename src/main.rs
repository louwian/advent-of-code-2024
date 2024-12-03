#[path = "day1/day1.rs"] mod day1;

fn main() {
    // You can now use functions or items from day1 module
    println!("Answer to day1 part1: {}", day1::solve_part1());
    println!("Answer to day1 part2: {}", day1::solve_part2());
}