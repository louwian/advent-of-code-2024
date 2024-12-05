use std::fs;
use regex::Regex;

pub fn solve_part1() -> i32 {
    
    let text: String = ingest_txt("./src/day3/data.txt");

    find_mul_equations_answer(&text).iter().sum()
}

pub fn solve_part2() -> i32 {
    let text: String = ingest_txt("./src/day3/data.txt");
    let re_pattern = Regex::new(r"(do\(\))|(don't\(\))|(mul\((\d{1,3}),(\d{1,3})\))").unwrap();

    let mut answers: Vec<i32> = Vec::new();

    let matches = re_pattern.captures_iter(&text);
    let mut status = true;

    for next in matches {
        if &next[0] == "don\'t()" {
            status = false;
        }
        if &next[0] == "do()" {
            status = true;
        }
        if next[0].contains("mul") && status {
            answers.push(find_mul_equations_answer(&next[0])[0]);
        }
        
    }

    answers.iter().sum()
}

fn find_mul_equations_answer(text: &str) -> Vec<i32> {
    // Find mul(x, y) equations

    let mut answers: Vec<i32> = Vec::new();
    
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut mul_equations: Vec<(String, i32, i32)> = Vec::new();
    for (_, [eqt, dig1, dig2]) in re.captures_iter(&text).map(|c| c.extract()) {
        mul_equations.push((eqt.to_owned(), dig1.parse::<i32>().unwrap(), dig2.parse::<i32>().unwrap()));
    }

    for (_, dig1, dig2) in mul_equations {
        let result = mul(dig1, dig2);
        answers.push(result);
    }
    
    answers
}

fn ingest_txt(file_path: &str) -> String {
    let contents: String = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    contents
}

fn mul(x: i32, y: i32) -> i32 {
    x * y
}
