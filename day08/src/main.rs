use std::collections::HashMap;

use day08::day08;

const DATA: &str = include_str!("../data/data.txt");

fn part1(antennas: &HashMap<char, Vec<day08::Point>>, map_w: i64, map_h: i64) {
    let output = day08::solve_part_1(antennas, map_w, map_h);
    println!("Part 1: {output}");
}

fn part2(antennas: &HashMap<char, Vec<day08::Point>>, map_w: i64, map_h: i64) {
    let output = day08::solve_part_2(antennas, map_w, map_h);
    println!("Part 2: {output}");
}

fn main() {
    let (antennas, (map_w, map_h)) = day08::read_data_from_string(&DATA.to_string());
    part1(&antennas, map_w, map_h);
    part2(&antennas, map_w, map_h);
}
