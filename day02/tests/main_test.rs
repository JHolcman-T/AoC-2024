use day02::day02;

const TEST_DATA: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

const PARSED_TEST_DATA: [[i64; 5]; 6] = [
    [7, 6, 4, 2, 1],
    [1, 2, 7, 8, 9],
    [9, 7, 6, 2, 1],
    [1, 3, 2, 4, 5],
    [8, 6, 4, 4, 1],
    [1, 3, 6, 7, 9],
];

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = day02::read_data_from_string(TEST_DATA.to_string());
    assert_eq!(output, PARSED_TEST_DATA.to_vec());
}

#[test]
fn test_check_allowed_distance_positive() {
    let output = day02::check_allowed_distance(1, 3);
    assert_eq!(output, true);
}

#[test]
fn test_check_allowed_distance_negative_greater() {
    let output = day02::check_allowed_distance(1, 5);
    assert_eq!(output, false);
}

#[test]
fn test_check_allowed_distance_negative_equal() {
    let output = day02::check_allowed_distance(1, 1);
    assert_eq!(output, false);
}

#[test]
fn test_categorize_records() {
    let output =
        day02::categorize_records(&PARSED_TEST_DATA.map(|record| record.to_vec()).to_vec());
    assert_eq!(output, vec![true, false, false, false, false, true]);
}

#[test]
fn test_categorize_records_with_errors() {
    let output = day02::categorize_records_with_errors(
        &PARSED_TEST_DATA.map(|record| record.to_vec()).to_vec(),
    );
    assert_eq!(output, vec![true, false, false, true, true, true]);
}
