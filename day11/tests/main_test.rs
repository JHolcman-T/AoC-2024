use day11::day11;

const TEST_DATA: &str = "125 17";

// const PARSED_TEST_DATA;

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = day11::read_data_from_string(TEST_DATA.to_string());
    let expected: [i64; 2] = [125, 17];
    assert_eq!(output, expected);
}
#[test]
fn test_solve_part1() {
    let data = day11::read_data_from_string(TEST_DATA.to_string());
    let output = day11::solve_part1(&data);
    let expected: i64 = 55312;
    assert_eq!(output, expected);
}
#[test]
fn test_solve_part2() {
    let data = day11::read_data_from_string(TEST_DATA.to_string());
    let output = day11::solve_part2(&data);
    let expected: i64 = 65601038650482;
    assert_eq!(output, expected);
}
