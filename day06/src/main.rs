use day06::day06;

const DATA: &str = include_str!("../data/data.txt");

fn part1(data: &Vec<Vec<char>>) {
    let output = day06::solve_part_1(data);
    println!("Part 1: {output}");
}

fn part2(data: &Vec<Vec<char>>) {
    let output = day06::solve_part_2(data);
    println!("Part 2: {output}");
}

fn main() {
    let parsed_data: Vec<Vec<char>> = day06::read_data_from_string(DATA);
    part1(&parsed_data);
    part2(&parsed_data);
}
