use ::day11::day11::{solve_part1, solve_part2};
use day11::day11;

const DATA: &str = include_str!("../data/data.txt");

fn part1(data: &Vec<i64>) {
    let output = solve_part1(data);
    println!("Part 1: {output}");
}

fn part2(data: &Vec<i64>) {
    let output = solve_part2(data);
    println!("Part 2: {output}");
}

fn main() {
    let parsed_data = day11::read_data_from_string(DATA.to_string());
    part1(&parsed_data);
    part2(&parsed_data);
}
