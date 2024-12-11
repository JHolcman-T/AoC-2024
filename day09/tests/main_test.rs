use day09::day09;

const TEST_DATA: &str = "2333133121414131402";

const PARSED_TEST_DATA: [&str; 42] = [
    "0", "0", ".", ".", ".", "1", "1", "1", ".", ".", ".", "2", ".", ".", ".", "3", "3", "3", ".",
    "4", "4", ".", "5", "5", "5", "5", ".", "6", "6", "6", "6", ".", "7", "7", "7", ".", "8", "8",
    "8", "8", "9", "9",
];

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = day09::read_data_from_string(TEST_DATA.to_string());
    assert_eq!(output, PARSED_TEST_DATA.to_vec());
}

#[test]
fn test_solve_part_1() {
    let data = day09::read_data_from_string(TEST_DATA.to_string());
    let output = day09::solve_part_1(&data);
    assert_eq!(output, 1928);
}

#[test]
fn test_solve_part_2() {
    let data = day09::read_data_from_string(TEST_DATA.to_string());
    let output = day09::solve_part_2(&data);
    assert_eq!(output, 2858);
}
