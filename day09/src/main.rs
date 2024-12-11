use day09::day09;

const DATA: &str = include_str!("../data/data.txt");

fn part1(data: &Vec<String>) {
    let output = day09::solve_part_1(data);
    println!("Part 1: {output}");
}

fn part2(data: &Vec<String>) {
    let output = day09::solve_part_2(data);
    println!("Part 2: {output}");
}

fn main() {
    let parsed_data = day09::read_data_from_string(DATA.to_string());
    part1(&parsed_data);
    part2(&parsed_data);
}
