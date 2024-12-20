use std::fs;

pub fn solve_part1() -> i64 {

    let input = ingest_txt("src/day9/data.txt");

    // Parse the disk map into file blocks
    let mut blocks = parse_disk_map(&input);

    // Simulate compaction
    compact_blocks(&mut blocks);

    // Calculate and return the checksum
    calculate_checksum(&blocks)
}

pub fn solve_part2() -> i64 {

    0
}

fn ingest_txt(file_path: &str) -> String {
    let contents: String = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    contents.trim().to_string()
}

fn parse_disk_map(disk_map: &str) -> Vec<Option<usize>> {
    let chars: Vec<usize> = disk_map.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut blocks = Vec::new();
    let mut file_id = 0;

    for (i, &len) in chars.iter().enumerate() {
        if i % 2 == 0 {
            // File block
            blocks.extend(vec![Some(file_id); len]);
            file_id += 1;
        } else {
            // Free space
            blocks.extend(vec![None; len]);
        }
    }

    blocks
}

fn compact_blocks(blocks: &mut Vec<Option<usize>>) {

    loop {
        let (space_index, is_space) = space_exists(blocks);
        let digit_index = first_digit(&blocks);
        if is_space && digit_index.is_some() {
            let (space_index, digit_index) = (space_index, digit_index.unwrap());
            blocks.swap(space_index, digit_index);
        } else {
            break;
        }
    }
}

fn calculate_checksum(blocks: &Vec<Option<usize>>) -> i64 {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, &file_id)| file_id.map(|id| pos as i64 * id as i64))
        .sum()
}

fn space_exists(blocks: &Vec<Option<usize>>) -> (usize, bool) {
    let mut space: (usize, bool) = (0, false);
    let mut first_null: usize = 0;
    let mut values = blocks.iter().enumerate();
    let (mut prev_index, mut prev_value) = values.next().unwrap();
    for (i, value) in values {
        if value.is_none() && first_null == 0 {
            first_null = i;
        }
        if value.is_some() && prev_value.is_none() {
            space = (prev_index, true);
            break;
        } else {
            (prev_index, prev_value) = (i, value)
        }
    }
    (first_null, space.1)
}

fn first_digit(blocks: &Vec<Option<usize>>) -> Option<usize> {

    let mut index: Option<usize> = None;

    for i in (0..blocks.len()).rev() {
        if blocks[i].is_some() {
            index = Some(i);
            break
        }
    }

    index
}