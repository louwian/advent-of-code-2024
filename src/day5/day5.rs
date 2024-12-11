use std::{collections::HashMap, fs};

pub fn solve_part1() -> i32 {
    
    let rules = ingest_txt("src/day5/rules.txt");
    let input = ingest_txt("src/day5/input.txt");

    let rules = process_rules(rules);
    let input = process_input(input);
    
    let rules_map = create_rules_hashmap(rules);

    let (valid_input, invalid_input) = get_valid_inputs(input, rules_map);
    sum_middle_values(valid_input)
}

pub fn solve_part2() -> i32 {
    
    let rules = ingest_txt("src/day5/rules.txt");
    let input = ingest_txt("src/day5/input.txt");

    let rules = process_rules(rules);
    let input = process_input(input);
    
    let rules_map = create_rules_hashmap(rules);

    let (valid_input, invalid_input) = get_valid_inputs(input, rules_map.clone());

    let fixed_input = fix_invalid_input(invalid_input, rules_map);

    sum_middle_values(fixed_input)
}

fn ingest_txt(file_path: &str) -> String {
    let contents: String = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    contents
}

fn process_rules(txt: String) -> Vec<(String, String)> {
    let mut rules: Vec<(String, String)> = Vec::new();
    for line in txt.lines() {
        let parts: Vec<&str> = line.split("|").collect();
        if parts.len() == 2 {
            rules.push((parts[0].to_string(), parts[1].to_string()));
        }
    }
    rules

}

fn process_input(txt: String) -> Vec<Vec<String>> {
    let mut values: Vec<Vec<String>> = Vec::new(); 
    for line in txt.lines() {
        let mut inner_values: Vec<String> = Vec::new();
        for value in line.split(",") {
            inner_values.push(value.to_string());
        }
        values.push(inner_values);
    }
    values
}

fn create_rules_hashmap(rules: Vec<(String, String)>) -> HashMap<String, HashMap<String, u8>> {
    let mut hashmap: HashMap<String, HashMap<String, u8>> = HashMap::new();
    for rule in rules {
        if hashmap.contains_key(&rule.0) {
            let mut values = hashmap.get_mut(&rule.0).unwrap();
            values.insert(rule.1, 1);
        } else {
            let mut values: HashMap<String, u8> = HashMap::new();
            values.insert(rule.1, 1);
            hashmap.insert(rule.0, values);
        }
    }
    hashmap
}

fn get_valid_inputs(input: Vec<Vec<String>>, rules_map: HashMap<String, HashMap<String, u8>>) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let mut valid_input: Vec<Vec<String>> = Vec::new();
    let mut invalid_input: Vec<Vec<String>> = Vec::new();

    for line in input {
        let length = line.len();
        let mut valid = true;
        for start in 0..length-1 {
            for index in start..length-1 {
                let rule = (line[start].to_string(), line[index+1].to_string());
                if !rules_map.contains_key(&rule.0) {
                    valid = false;
                    break;
                }
                if !rules_map.get(&rule.0).unwrap().contains_key(&rule.1) {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            valid_input.push(line);
        }
        else {
            invalid_input.push(line);
        }
    }

    (valid_input, invalid_input)

}

fn fix_invalid_input(invalid_input: Vec<Vec<String>>, rules_map: HashMap<String, HashMap<String, u8>>) -> Vec<Vec<String>> {
    let mut fixed_input: Vec<Vec<String>> = Vec::new();
    for line in invalid_input {
        let mut fixed_line: Vec<String> = line.clone();
        let length = line.len();
        let mut valid = false;

        let mut _start: usize = 0;
        let mut _index: usize = 0;

        while !valid {
            valid = true;

            fixed_line.swap(_index+1, _start);
            
            for start in _start..length-1 {
                _start = start.clone();
                for index in _start..length-1 {
                    _index = index.clone();
                    let rule = (fixed_line[start].to_string(), fixed_line[index+1].to_string());
                    if !rules_map.contains_key(&rule.0) {
                        valid = false;
                        break;
                    }
                    if !rules_map.get(&rule.0).unwrap().contains_key(&rule.1) {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }
        }

        fixed_input.push(fixed_line);

    }
    fixed_input
}

fn sum_middle_values(input: Vec<Vec<String>>) -> i32 {
    let mut sum = 0;
    for line in input {
        if line.len() > 2 {
            let middle_index = (line.len() + 1) / 2;
            if let Ok(value) = line[middle_index-1].parse::<i32>() {
                sum += value;
            }
        }
    }
    sum
}