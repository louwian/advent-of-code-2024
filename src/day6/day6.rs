use std::{collections::HashMap, fs};

pub fn solve_part1() -> i32 {

    let grid = ingest_txt("src/day6/test.txt");
    let grid = process_grid(grid);
    let grid_size = (grid.len(), grid[0].len());
    let mut grid_visits = vec![vec![0; grid_size.1]; grid_size.0];
    let start_position = find_start_position(&grid);

    let mut direction_map = HashMap::new();
    direction_map.insert('^', '>');
    direction_map.insert('>', 'v');
    direction_map.insert('v', '<');
    direction_map.insert('<', '^');
    
    // Fast forward moves
    let mut current_position = start_position;
    record_visit(&mut grid_visits, current_position);
    while is_within_grid(current_position, grid_size) {
        let backup_position = current_position.clone();
        match current_position.0 {
            '^' => current_position.1 -= 1,
            '>' => current_position.2 += 1,
            '<' => current_position.2 -= 1,
            'v' => current_position.1 += 1,
            _ => (),
        }

        if is_within_grid(current_position, grid_size) {
            let grid_value = get_grid_value(&grid, (current_position.1, current_position.2));
            match grid_value {
                '.' => record_visit(&mut grid_visits, current_position),
                '#' => {
                    current_position = backup_position;
                    current_position.0 = direction_map.get(&current_position.0).unwrap().clone();
                },
                '^' => record_visit(&mut grid_visits, current_position),
                _ => break,
            }
            ;
        }
    } 

    print_grid(&grid_visits);

    sum_visits(&grid_visits)
}

pub fn solve_part2() -> i32 {

    0
}

fn ingest_txt(file_path: &str) -> Vec<String> {
    let contents: String = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let contents = contents.lines().map(|x| x.to_string()).collect();
    contents
}

fn process_grid(grid: Vec<String>) -> Vec<Vec<char>> {
    let mut grid = grid.iter().map(|x| x.chars().collect()).collect();
    grid
}

fn find_start_position(grid: &Vec<Vec<char>>) -> (char, usize, usize) {
    let mut start_position = ('^', 0, 0);
    for (i, row) in grid.iter().enumerate() {
        if let Some(j) = row.iter().position(|&c| c == '^') {
            start_position = ('^', i, j);
        }
        if let Some(j) = row.iter().position(|&c| c == '>') {
            start_position = ('>', i, j);
        }
        if let Some(j) = row.iter().position(|&c| c == '<') {
            start_position = ('<', i, j);
        }
        if let Some(j) = row.iter().position(|&c| c == 'v') {
            start_position = ('v', i, j);
        }
    }
    start_position
}

fn record_visit(grid_visits: &mut Vec<Vec<i32>>, current_position: (char, usize, usize)) {
    grid_visits[current_position.1][current_position.2] = 1;
}

fn is_within_grid(current_position: (char, usize, usize), grid_size: (usize, usize)) -> bool {
    let mut within = true;

    if current_position.1 < 0 || current_position.1 >= grid_size.0 {
        within = false;
    }
    if current_position.2 < 0 || current_position.2 >= grid_size.1 {
        within = false;
    }

    within
}

fn get_grid_value(grid: &Vec<Vec<char>>, position: (usize, usize)) -> char {
    grid[position.0][position.1]
}

fn print_grid(grid: &Vec<Vec<i32>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn sum_visits(grid_visits: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for row in grid_visits {
        for c in row {
            sum += c;
        }
    }
    sum
}