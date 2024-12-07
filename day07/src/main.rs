use day07::day07;

const DATA: &str = include_str!("../data/data.txt");

fn part1(data: &Vec<(i64, Vec<i64>)>) {
    let output = day07::solve_part_1(data);
    println!("Part 1: {output}");
}

fn part2(data: &Vec<(i64, Vec<i64>)>) {
    let output = day07::solve_part_2(data);
    println!("Part 2: {output}");
}

fn main() {
    let parsed_data = day07::read_data_from_string(DATA.to_string());
    part1(&parsed_data);
    part2(&parsed_data);
}
