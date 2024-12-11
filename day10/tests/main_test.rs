use day10::day10;

const TEST_DATA: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

// const PARSED_TEST_DATA;

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = day10::read_data_from_string(TEST_DATA.to_string());
    let expected = [
        ['8', '9', '0', '1', '0', '1', '2', '3'],
        ['7', '8', '1', '2', '1', '8', '7', '4'],
        ['8', '7', '4', '3', '0', '9', '6', '5'],
        ['9', '6', '5', '4', '9', '8', '7', '4'],
        ['4', '5', '6', '7', '8', '9', '0', '3'],
        ['3', '2', '0', '1', '9', '0', '1', '2'],
        ['0', '1', '3', '2', '9', '8', '0', '1'],
        ['1', '0', '4', '5', '6', '7', '3', '2'],
    ];
    assert_eq!(output, expected);
}

#[test]
fn test_solve_part1_part2() {
    let data = day10::read_data_from_string(TEST_DATA.to_string());
    let expected_part1: usize = 36;
    let expected_part2: usize = 81;
    let (out_part1, out_part2) = day10::solve_part1_part_2(&data);
    assert_eq!(out_part1, expected_part1);
    assert_eq!(out_part2, expected_part2);
}
