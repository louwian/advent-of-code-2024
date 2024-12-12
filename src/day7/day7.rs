use std::fs;
use itertools::{repeat_n, Itertools};

pub fn solve_part1() -> i64 {

    let input = ingest_txt("src/day7/data.txt");
    let input = process_text(input);

    let mut valid_answers: Vec<i64> = Vec::new();
    
    for (answer, digits) in input {
        if is_valid(answer, digits, vec!['+', '*']) {
            valid_answers.push(answer)
        }
    }

    valid_answers.iter().sum()
}

pub fn solve_part2() -> i64 {

    let input: Vec<String> = ingest_txt("src/day7/data.txt");
    let input: Vec<(i64, Vec<i64>)> = process_text(input);
    let mut invalid_input: Vec<(i64, Vec<i64>)> = Vec::new();

    let mut valid_answers: Vec<i64> = Vec::new();
    
    for (answer, digits) in input {
        if is_valid(answer, digits.clone(), vec!['+', '*']) {
            valid_answers.push(answer)
        } else {
            invalid_input.push((answer, digits));
        }     
    }

    for (answer, digits) in invalid_input {
        if is_valid(answer, digits.clone(), vec!['+', '*', '|']) {
            valid_answers.push(answer)
        }
    }

    valid_answers.iter().sum()
}

fn ingest_txt(file_path: &str) -> Vec<String> {
    let contents: String = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let contents = contents.lines().map(|x| x.to_string()).collect();
    contents
}

fn process_text(text: Vec<String>) -> Vec<(i64, Vec<i64>)> {

    let mut processed_text: Vec<(i64, Vec<i64>)> = Vec::new();

    for line in text {
        let mut input_values: Vec<i64> = Vec::new();
        let mut split_line: Vec<&str> = line.split(":").collect();
        let answer = split_line[0].parse::<i64>().unwrap();
        
        let mut split_values: Vec<&str> = split_line[1].trim().split(" ").collect();
        for value in split_values {
            input_values.push(value.parse::<i64>().unwrap());
        }
        processed_text.push((answer, input_values));
    }

    processed_text
}

fn get_operator_permutations(lngth: usize, items: Vec<char>) -> Vec<Vec<char>> {
    let mut perms: Vec<Vec<char>> = Vec::new();;
    let items = items.iter();
    for perm in repeat_n(items, lngth).multi_cartesian_product() {
        perms.push(perm.into_iter().cloned().collect());
    }
    perms.clone()
}

fn is_valid(answer: i64, digits: Vec<i64>, operators: Vec<char>) -> bool {
    let mut valid = false;
    let operator_permutations = get_operator_permutations(digits.len() - 1, operators.clone());
    for perm in operator_permutations {

        let mut result = digits[0];

        for (i, operator) in perm.iter().enumerate() {
            match operator {
                '+' => result += digits[i + 1],
                '*' => result *= digits[i + 1],
                '|' => result = join_ints(result, digits[i + 1]),
                _ => (),
            }
            if result > answer {
                break;
            }
        }

        if result == answer {
            valid = true;
            break;
        }
    }
    valid
}

fn join_ints(i: i64, j: i64) -> i64 {
    let mut answer  = String::new();
    answer.push_str(&i.to_string());
    answer.push_str(&j.to_string());

    answer.parse::<i64>().unwrap()
}