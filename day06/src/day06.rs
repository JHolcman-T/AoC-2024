use std::{
    collections::{HashSet, VecDeque},
    env::current_exe,
};

pub fn read_data_from_string(input_string: &'static str) -> Vec<Vec<char>> {
    return input_string
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
}

pub fn get_start_point(data: &Vec<Vec<char>>) -> (i64, i64) {
    for (y_axis, line) in data.iter().enumerate() {
        for (x_axis, char) in line.iter().enumerate() {
            if *char == '^' {
                return (x_axis as i64, y_axis as i64);
            }
        }
    }
    return (0, 0);
}

pub fn solve_part_1(data: &Vec<Vec<char>>) -> usize {
    let mut current_position: (i64, i64) = get_start_point(data);
    // (x_axis, y_axis)
    let directions: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut direction_index: usize = 0;
    let width = data[0].len();
    let height = data.len();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    loop {
        visited.insert(current_position);

        let next_pos = (
            current_position.0 + directions[direction_index].0,
            current_position.1 + directions[direction_index].1,
        );

        if next_pos.0 >= width as i64
            || next_pos.0 < 0
            || next_pos.1 >= height as i64
            || next_pos.1 < 0
        {
            break;
        }

        let ch = data[next_pos.1 as usize][next_pos.0 as usize];

        if ch == '#' {
            direction_index = (direction_index + 1) % 4;
            continue;
        }

        current_position = next_pos;
    }

    return visited.len();
}

pub fn is_cycle(data: &Vec<Vec<char>>, start_position: (i64, i64)) -> bool {
    let mut current_position = start_position;
    let directions: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut direction_index: usize = 0;
    let width = data[0].len();
    let height = data.len();
    let mut visited: HashSet<Vec<i64>> = HashSet::new();

    loop {
        let next_pos = (
            current_position.0 + directions[direction_index].0,
            current_position.1 + directions[direction_index].1,
        );

        if next_pos.0 >= width as i64
            || next_pos.0 < 0
            || next_pos.1 >= height as i64
            || next_pos.1 < 0
        {
            break;
        }

        let ch = data[next_pos.1 as usize][next_pos.0 as usize];

        if ch == '#' {
            if visited.contains(&vec![
                current_position.0,
                current_position.1,
                direction_index as i64,
            ]) {
                return true;
            }

            visited.insert(vec![
                current_position.0,
                current_position.1,
                direction_index as i64,
            ]);

            direction_index = (direction_index + 1) % 4;
            continue;
        }

        current_position = next_pos;
    }
    return false;
}

pub fn solve_part_2(data: &Vec<Vec<char>>) -> usize {
    let start_position: (i64, i64) = get_start_point(data);
    let mut cycles: usize = 0;
    // (x_axis, y_axis)
    let mut sim_data = data.clone();
    for row in 0..data.len() {
        for column in 0..data[0].len() {
            let prev_char = sim_data[row][column];

            // don't place things at guard starting point
            if (column as i64, row as i64) == start_position || prev_char == '#' {
                continue;
            }

            sim_data[row][column] = '#';

            if is_cycle(&sim_data, start_position) {
                cycles += 1;
            }
            sim_data[row][column] = prev_char;
        }
    }

    return cycles;
}
