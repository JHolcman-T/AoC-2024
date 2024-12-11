use day10::day10;

const DATA: &str = include_str!("../data/data.txt");

fn part1(output: usize) {
    println!("Part 1: {output}");
}

fn part2(output: usize) {
    println!("Part 2: {output}");
}

fn main() {
    let parsed_data = day10::read_data_from_string(DATA.to_string());
    let (out_part1, out_part2) = day10::solve_part1_part_2(&parsed_data);
    part1(out_part1);
    part2(out_part2);
}
