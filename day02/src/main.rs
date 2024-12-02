use day02::day02;

const DATA: &str = include_str!("../data/data.txt");

fn part1(records: &Vec<Vec<i64>>) {
    let valid_records = day02::categorize_records(records);
    let valid_records_count = valid_records.iter().filter(|value| **value == true).count();
    println!("Part 1: {valid_records_count}");
}

fn part2(records: &Vec<Vec<i64>>) {
    let valid_records = day02::categorize_records_with_errors(records);
    let valid_records_count = valid_records.iter().filter(|value| **value == true).count();
    println!("Part 2: {valid_records_count}");
}

fn main() {
    let records = day02::read_data_from_string(DATA.to_string());
    part1(&records);
    part2(&records);
}
