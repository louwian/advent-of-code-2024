use std::fs;
use regex::Regex;

pub fn solve_part1() -> i32 {

    let re_pattern1 = Regex::new(r"(XMAS)|(SAMX)").unwrap();
    let re_pattern2 = Regex::new(r"(XMASAMX)|(SAMXMAS)").unwrap();

    let text: String = ingest_txt("./src/day4/data.txt");
    let vertical_text = process_txt_vertical(text.clone());
    let horizontal_text = process_txt_horizontal(text.clone());
    let (mut pos_upper_diagonal_text, mut pos_lower_diagonal_text) = process_pos_diagonal(text.clone());
    pos_upper_diagonal_text = process_txt_vertical(pos_upper_diagonal_text.join("\n"));
    pos_lower_diagonal_text = process_txt_vertical(pos_lower_diagonal_text.join("\n"));
    let (mut neg_upper_diagonal_text, mut neg_lower_diagonal_text ) = process_neg_diagonal(text.clone());
    neg_upper_diagonal_text = process_txt_vertical(neg_upper_diagonal_text.join("\n"));
    neg_lower_diagonal_text = process_txt_vertical(neg_lower_diagonal_text.join("\n"));

    let mut all_texts = Vec::new();
    all_texts.extend(vertical_text);
    all_texts.extend(horizontal_text);
    all_texts.extend(pos_upper_diagonal_text);
    all_texts.extend(pos_lower_diagonal_text);
    all_texts.extend(neg_upper_diagonal_text);
    all_texts.extend(neg_lower_diagonal_text);
    

    let mut count = 0;
    for line in all_texts {
        let matches = re_pattern1.captures_iter(&line);
        for mtch in matches {
            count += 1
        }
        let matches = re_pattern2.captures_iter(&line);
        for mtch in matches {
            count += 1
        }
    }
    
    count

}

fn ingest_txt(file_path: &str) -> String {
    let contents: String = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    contents
}

fn process_txt_horizontal(text: String) -> Vec<String> {
    let lines: Vec<&str> = text.lines().collect();
    let mut horizontal_lines: Vec<String> = Vec::new();
    for line in lines {
        horizontal_lines.push(line.to_string());
    }
    horizontal_lines
}

fn process_txt_vertical(text: String) -> Vec<String> {
    let mut vertical_lines: Vec<String> = Vec::new();
    let lines: Vec<&str> = text.lines().collect();
    for i in 0..lines[0].len() {
        let mut vertical_line = String::new();
        for line in &lines {
            if let Some(c) = line.chars().nth(i) {
                vertical_line.push(c);
            }
        }
        vertical_lines.push(vertical_line);
    }
    vertical_lines
}

fn process_pos_diagonal(text: String) -> (Vec<String>, Vec<String>) {

    let lines: Vec<&str> = text.lines().collect();
    let len = lines[0].len();
    let mut upper_diagonal_lines: Vec<String> = Vec::new();
    let mut lower_diagonal_lines: Vec<String> = Vec::new();
    for i in 0..2 {
        for j in i..lines.len() {
            let row_len = lines[j].len();
            let mut row: String;
            row = lines.clone()[j].to_string();
            if i == 0 {
                row = row[j..row_len].to_string();
                row.push_str(&" ".repeat(j));
                upper_diagonal_lines.push(row.clone());
            }
            else {
                row = row[0..j].to_string();
                row = " ".repeat(row_len-j) + &row;
                lower_diagonal_lines.push(row.clone());
            }
        }
    }
    (upper_diagonal_lines, lower_diagonal_lines)
}

fn process_neg_diagonal(text: String) -> (Vec<String>, Vec<String>) {

    let lines: Vec<&str> = text.lines().collect();
    let len = lines[0].len();
    let mut upper_diagonal_lines: Vec<String> = Vec::new();
    let mut lower_diagonal_lines: Vec<String> = Vec::new();
    for i in 0..2 {
        for j in i..lines.len() {
            let row_len = lines[j].len();
            let mut row: String;
            row = lines.clone()[j].to_string();
            if i == 0 {
                row = row[0..row_len-j].to_string();
                row = " ".repeat(j) + &row;
                upper_diagonal_lines.push(row.clone());
            }
            else {
                row = row[row_len-j..row_len].to_string();
                row.push_str(&" ".repeat(row_len-j));
                lower_diagonal_lines.push(row.clone());
            }
        }
    }
    (upper_diagonal_lines, lower_diagonal_lines)
}