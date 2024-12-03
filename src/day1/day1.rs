use std::fs;

pub fn solve_part1() -> i32 {

    let mut diffs: Vec<i32> = Vec::new();

    let parsed_data = parse_data();
    let mut list1 = parsed_data.0;
    let mut list2 = parsed_data.1;
    list1.sort();
    list2.sort();

    let merged_list: Vec<(i32, i32)> = list1.iter().zip(list2.iter()).map(|(a, b)| (*a, *b)).collect();
    
    for (a, b) in merged_list {
        let diff: i32 = (a - b).abs();
        diffs.push(diff as i32);
    }

    return diffs.iter().sum();
}

pub fn solve_part2() -> i32 {

    let mut result: Vec<i32> = Vec::new();
    let mut counts: Vec<(i32, i32)> = Vec::new();

    let parsed_data = parse_data();
    let list1 = parsed_data.0;
    let list2 = parsed_data.1;
    
    for &item in &list1 {
        let count = list2.iter().filter(|&&x| x == item).count() as i32;
        counts.push((item, count));
        result.push(item*count);
    }

    return result.iter().sum();
}

fn ingest_txt(file_path: &str) -> Vec<String> {
    let contents: String = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");
    contents.lines().map(|s| s.to_string()).collect()

}

fn process_text(data: Vec<String>) -> Vec<(i32, i32)> {
    let processed_data_str: Vec<(i32, i32)> = data.into_iter()
        .map(|s| {
            let mut parts = s.split_whitespace();
            let first: i32 = parts.next().unwrap().trim().parse().expect("Unable to parse");
            let second: i32 = parts.next().unwrap().trim().parse().expect("Unable to parse");
            (first, second)
        }).collect();

    processed_data_str
}

fn parse_data() -> (Vec<i32>, Vec<i32>) {

    let file_path: &str = "./src/day1/data.txt";
    let data: Vec<String> = ingest_txt(&file_path);
    let processed_data: Vec<(i32, i32)> = process_text(data);
    let list1: Vec<i32> = processed_data.clone().into_iter().map(|(a, _)| a).collect();
    let list2: Vec<i32> = processed_data.into_iter().map(|(_, b)| b).collect();

    return (list1, list2);
}