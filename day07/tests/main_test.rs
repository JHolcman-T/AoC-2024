use day07::day07;

const TEST_DATA: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

// const PARSED_TEST_DATA;

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = day07::read_data_from_string(TEST_DATA.to_string());
    let expected: [(i64, Vec<i64>); 9] = [
        (190, vec![10, 19]),
        (3267, vec![81, 40, 27]),
        (83, vec![17, 5]),
        (156, vec![15, 6]),
        (7290, vec![6, 8, 6, 15]),
        (161011, vec![16, 10, 13]),
        (192, vec![17, 8, 14]),
        (21037, vec![9, 7, 18, 13]),
        (292, vec![11, 6, 16, 20]),
    ];
    assert_eq!(output, expected);
}

#[test]
fn test_is_valid_calculation_part_1() {
    let data = day07::read_data_from_string(TEST_DATA.to_string());
    let output = day07::solve_part_1(&data);

    assert_eq!(output, 3749);
}

#[test]
fn test_is_valid_calculation_part_2() {
    let data = day07::read_data_from_string(TEST_DATA.to_string());
    let output = day07::solve_part_2(&data);

    assert_eq!(output, 11387);
}
