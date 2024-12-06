use std::collections::HashSet;

use day05::day05;

const TEST_DATA: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

// const PARSED_TEST_DATA;

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = day05::read_data_from_string(TEST_DATA.to_string());
    println!("{:?}", output);
    // assert_eq!(output, PARSED_TEST_DATA.to_vec());
}

#[test]
fn test_calculate_valid_middle_numbers_sum() {
    let parsed_data = day05::read_data_from_string(TEST_DATA.to_string());
    let output = day05::calculate_valid_middle_numbers_sum(&parsed_data);
    assert_eq!(output, 143);
}

#[test]
fn test_fix_order() {
    let parsed_data = day05::read_data_from_string(TEST_DATA.to_string());
    let update_order = vec![61, 13, 29];
    let output = day05::fix_order(&parsed_data.rules, &update_order);
    assert_eq!(output, vec![61, 29, 13,]);
}

#[test]
fn test_get_incorrect_ordered_updates() {
    let parsed_data = day05::read_data_from_string(TEST_DATA.to_string());
    let expected_update_order: Vec<Vec<usize>> = vec![
        vec![75, 97, 47, 61, 53],
        vec![61, 13, 29],
        vec![97, 13, 75, 29, 47],
    ];
    let output = day05::get_incorrect_ordered_updates(&parsed_data);
    assert_eq!(output, expected_update_order);
}

#[test]
fn test_calculate_middle_numbers_sum_from_corrected_update_orders() {
    let parsed_data = day05::read_data_from_string(TEST_DATA.to_string());
    let output = day05::calculate_middle_numbers_sum_from_corrected_update_orders(&parsed_data);
    assert_eq!(output, 123);

    // correct: 97,75,47,29,13
    // 97,13,75,29,47
    // 97,75,13,29,47
    // 97,75,29,13,47
    // 97,75,29,47,13
    // needs multiple passes
}
