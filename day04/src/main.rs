use day04::day04;

const DATA: &str = include_str!("../data/data.txt");

fn part1(input_data: Vec<Vec<char>>) {
    let output = day04::find_words_part1(input_data);
    println!("Part 1: {output}");
}

fn part2(input_data: Vec<Vec<char>>) {
    let output = day04::find_words_part2(input_data);
    println!("Part 2: {output}");
}

fn main() {
    let parsed_data = day04::read_data_from_string(DATA.to_string());
    part1(parsed_data.clone());
    part2(parsed_data.clone());
}
