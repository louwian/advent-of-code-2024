use std::fs;

pub fn solve_part1() -> u32 {
    let mut count: u32 = 0;

    let data: Vec<Vec<u8>> = ingest_txt("./src/day2/data.txt");
    let mut valid_reports: Vec<(bool, bool)> = Vec::new();

    valid_reports = data.iter().map(|report| (check_ascending(report), check_diffs(report))).collect();

    for report in valid_reports {
        if report.0 && report.1 {
            count += 1;
        }
    }

    count
}

pub fn solve_part2() -> u32 {
    let mut count: u32 = 0;

    let data: Vec<Vec<u8>> = ingest_txt("./src/day2/data.txt");
    let mut valid_reports: Vec<(bool, bool)> = Vec::new();

    valid_reports = data.iter().map(|report| (check_ascending(report), check_diffs(report))).collect();

    for (i, report) in valid_reports.iter().enumerate() {
        if report.0 && report.1 {
            count += 1;
        }
        else {
            for j in 0..data[i].len() {
                let mut temp = data[i].clone();
                temp.remove(j);
                if check_ascending(&temp) && check_diffs(&temp) {
                    count += 1;
                    break
                }
            }
        }
    }

    count
}

fn ingest_txt(file_path: &str) -> Vec<Vec<u8>> {
    let contents: String = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    contents
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn check_ascending(report: &Vec<u8>) -> bool {
    let ascending = report.windows(2).all(|w| w[0] < w[1]);
    let descending = report.windows(2).all(|w| w[0] > w[1]);
    ascending || descending
}

fn check_diffs(report: &Vec<u8>) -> bool {
    let diffs: Vec<u8> = report.windows(2).map(|w| w[1].abs_diff(w[0])).collect();
    let checks = [
        diffs.iter().all(|&x| x >= 1),
        diffs.iter().all(|&x| x <= 3),
    ];
    checks.iter().all(|&x| x)
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_solve_part1() {
//         assert_eq!(solve_part1(), /* expected result */);
//     }
// }
