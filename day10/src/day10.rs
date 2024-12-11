use std::{collections::HashSet, usize};

pub fn read_data_from_string(input_string: String) -> Vec<Vec<char>> {
    return input_string
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
}

#[derive(Debug)]
pub struct Route {
    pub number: u8,
    pub position: (i64, i64),
    pub routes: Vec<Route>,
}

pub fn find_trails(
    map: &Vec<Vec<char>>,
    map_size: (i64, i64),
    current_number: u8,
    current_position: (i64, i64),
) -> (u8, (i64, i64), Vec<Route>, HashSet<(u8, (i64, i64))>, usize) {
    let next_number = current_number + 1;
    let (row, col) = current_position;
    let up = (row - 1, col);
    let down = (row + 1, col);
    let right = (row, col + 1);
    let left = (row, col - 1);

    let mut routes: Vec<Route> = vec![];
    let mut routes_count: usize = 0;
    let mut nines: HashSet<(u8, (i64, i64))> = HashSet::new();

    for (d_row, d_col) in [up, down, right, left] {
        if d_row < map_size.0 && d_row > -1 && d_col < map_size.1 && d_col > -1 {
            let grabbed_number = map[d_row as usize][d_col as usize];
            if grabbed_number.to_digit(10).unwrap() as u8 == next_number {
                if next_number == 9 {
                    routes.push(Route {
                        number: next_number,
                        position: (d_row, d_col),
                        routes: vec![],
                    });
                    nines.insert((next_number, (d_row, d_col)));
                } else {
                    let (out_number, out_position, out_routes, out_nines, out_routes_count) =
                        find_trails(map, map_size, next_number, (d_row, d_col));
                    if out_nines.len() != 0 {
                        nines.extend(out_nines);
                        routes.push(Route {
                            number: out_number,
                            position: out_position,
                            routes: out_routes,
                        });
                        routes_count += out_routes_count;
                    }
                }
            }
        }
    }

    if next_number == 9 {
        routes_count = routes.len();
    }

    return (
        current_number,
        current_position,
        routes,
        nines,
        routes_count,
    );
}

pub fn solve_part1_part_2(map: &Vec<Vec<char>>) -> (usize, usize) {
    let mut part_1: usize = 0;
    let mut part_2: usize = 0;
    let map_size = (map.len() as i64, map[0].len() as i64);
    for (row, line) in map.iter().enumerate() {
        for (col, char) in line.iter().enumerate() {
            if *char == '0' {
                let out = find_trails(map, map_size, 0, (row as i64, col as i64));
                part_1 += out.3.len();
                part_2 += out.4;
            }
        }
    }

    return (part_1, part_2);
}
