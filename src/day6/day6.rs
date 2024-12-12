use std::{collections::HashMap, fs};

pub fn solve_part1() -> i32 {

    let grid = ingest_txt("src/day6/data.txt");
    let grid = process_grid(grid);
    let grid_size = (grid.len(), grid[0].len());
    let mut grid_visits = vec![vec![0; grid_size.1]; grid_size.0];
    let mut grid_directions = vec![vec!['.'; grid_size.1]; grid_size.0];
    
    get_visited_positions(grid, &mut grid_visits, &mut grid_directions);
    
    print_grid(&grid_visits);

    sum_visits(&grid_visits)
}

pub fn solve_part2() -> i32 {

    let mut loop_count = 0;

    let grid = ingest_txt("src/day6/data.txt");
    let grid = process_grid(grid);
    let grid_size = (grid.len(), grid[0].len());
    let mut grid_visits = vec![vec![0; grid_size.1]; grid_size.0];
    let mut grid_directions = vec![vec!['.'; grid_size.1]; grid_size.0];
    
    get_visited_positions(grid.clone(), &mut grid_visits, &mut grid_directions);

    let obstacle_locations = find_visited_positions(&grid_visits);

    let grid_directions = vec![vec!['.'; grid_size.1]; grid_size.0];

    for (i, j) in obstacle_locations {
        let mut grid = grid.clone();
        grid[i][j] = '#';
        let is_loop = check_loop(grid, grid_directions.clone());
        if is_loop {
            loop_count += 1;
        }
    }

    loop_count
    
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

fn get_visited_positions(grid: Vec<Vec<char>>, grid_visits: &mut Vec<Vec<i32>>, grid_directions: &mut Vec<Vec<char>>) {

    let grid_size = (grid.len(), grid[0].len());

    let start_position = find_start_position(&grid);

    let mut direction_map = HashMap::new();
    direction_map.insert('^', '>');
    direction_map.insert('>', 'v');
    direction_map.insert('v', '<');
    direction_map.insert('<', '^');
    
    // Fast forward moves
    let mut current_position = start_position;
    record_visit(grid_visits, current_position);
    record_direction(grid_directions, current_position);
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
            let grid_value: char = get_grid_value(&grid, (current_position.1, current_position.2));
            match grid_value {
                '.' => {
                    record_visit(grid_visits, current_position);
                    record_direction(grid_directions, current_position);
                },
                '#' => {
                    current_position = backup_position;
                    current_position.0 = direction_map.get(&current_position.0).unwrap().clone();
                },
                '^' => {
                    record_visit(grid_visits, current_position);
                    record_direction(grid_directions, current_position);
                },
                _ => break,
            }
            ;
        }
    } 
}

fn check_loop(grid: Vec<Vec<char>>, grid_directions: Vec<Vec<char>>) -> bool {

    let mut grid_directions = grid_directions.clone();

    let mut is_loop = false;

    let grid_size = (grid.len(), grid[0].len());

    let start_position = find_start_position(&grid);

    let mut direction_map = HashMap::new();
    direction_map.insert('^', '>');
    direction_map.insert('>', 'v');
    direction_map.insert('v', '<');
    direction_map.insert('<', '^');
    
    // Fast forward moves
    let mut current_position = start_position;
    record_direction(&mut grid_directions, current_position);
    while is_within_grid(current_position, grid_size) {
        let backup_position = current_position.clone();
        match current_position.0 {
            '^' => {
                if current_position.1 == 0 {
                    break;
                }
                current_position.1 -= 1;
            },
            '>' => current_position.2 += 1,
            '<' => {
                if current_position.2 == 0 {
                    break;
                }
                current_position.2 -= 1;
            },
            'v' => current_position.1 += 1,
            _ => (),
        }

        if is_within_grid(current_position, grid_size) {
            let grid_value: char = get_grid_value(&grid, (current_position.1, current_position.2));
            let grid_direction = get_grid_value(&grid_directions, (current_position.1, current_position.2));
            if grid_direction == current_position.0 {
                is_loop = true;
                break;
            }
            match grid_value {
                '.' => {
                    record_direction(&mut grid_directions, current_position);
                },
                '#' => {
                    current_position = backup_position;
                    current_position.0 = direction_map.get(&current_position.0).unwrap().clone();
                },
                '^' => {
                    record_direction(&mut grid_directions, current_position);
                },
                _ => break,
            }
        }
    } 
    is_loop
}

fn record_visit(grid_visits: &mut Vec<Vec<i32>>, current_position: (char, usize, usize)) {
    grid_visits[current_position.1][current_position.2] = 1;
}

fn record_direction(grid_visits: &mut Vec<Vec<char>>, current_position: (char, usize, usize)) {
    grid_visits[current_position.1][current_position.2] = current_position.0;
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

fn print_grid<T: std::fmt::Display>(grid: &Vec<Vec<T>>) {
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

fn find_visited_positions(visited_positions: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for (i, row) in visited_positions.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == 1 {
                positions.push((i, j));
            }
        }
    }
    positions

}