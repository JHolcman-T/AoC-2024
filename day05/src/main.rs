use day05::day05;

const DATA: &str = include_str!("../data/data.txt");

fn part1(parsed_data: &day05::ParsedData) {
    let output: usize = day05::calculate_valid_middle_numbers_sum(parsed_data);
    println!("Part 1: {output}");
}

fn part2(parsed_data: &day05::ParsedData) {
    let output: usize =
        day05::calculate_middle_numbers_sum_from_corrected_update_orders(parsed_data);
    println!("Part 2: {output}");
}

fn main() {
    let parsed_data = day05::read_data_from_string(DATA.to_string());
    part1(&parsed_data);
    part2(&parsed_data);
}
