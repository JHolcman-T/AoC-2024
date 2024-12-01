use day01::day01;

const DATA: &str = include_str!("../data/data.txt");

fn part1(list_left: &Vec<i64>, list_right: &Vec<i64>) {
    let result = day01::sum_lists(&list_left, &list_right);

    println!("Part 1: {result}");
}

fn part2(list_left: &Vec<i64>, list_right: &Vec<i64>) {
    let result = day01::count_and_sum(&list_left, &list_right);

    println!("Part 2: {result}");
}

fn main() {
    let (list_left, list_right) = day01::read_data_from_string(DATA.to_string());
    part1(&list_left, &list_right);
    part2(&list_left, &list_right);
}
