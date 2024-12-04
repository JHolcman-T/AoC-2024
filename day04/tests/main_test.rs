use day04::day04;

const TEST_DATA: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

const PARSED_TEST_DATA: [[char; 10]; 10] = [
    //  0    1    2    3    4    5    6    7    8    9
    ['M', 'M', 'M', 'S', 'X', 'X', 'M', 'A', 'S', 'M'], // 0
    ['M', 'S', 'A', 'M', 'X', 'M', 'S', 'M', 'S', 'A'], // 1
    ['A', 'M', 'X', 'S', 'X', 'M', 'A', 'A', 'M', 'M'], // 2
    ['M', 'S', 'A', 'M', 'A', 'S', 'M', 'S', 'M', 'X'], // 3
    ['X', 'M', 'A', 'S', 'A', 'M', 'X', 'A', 'M', 'M'], // 4
    ['X', 'X', 'A', 'M', 'M', 'X', 'X', 'A', 'M', 'A'], // 5
    ['S', 'M', 'S', 'M', 'S', 'A', 'S', 'X', 'S', 'S'], // 6
    ['S', 'A', 'X', 'A', 'M', 'A', 'S', 'A', 'A', 'A'], // 7
    ['M', 'A', 'M', 'M', 'M', 'X', 'M', 'M', 'M', 'M'], // 8
    ['M', 'X', 'M', 'X', 'A', 'X', 'M', 'A', 'S', 'X'], // 9
];

const TEST_DATA_2: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = day04::read_data_from_string(TEST_DATA.to_string());
    assert_eq!(output, PARSED_TEST_DATA.to_vec());
}

#[test]
fn test_part_1() {
    let parsed_data = day04::read_data_from_string(TEST_DATA.to_string());
    let output = day04::find_words_part1(parsed_data);
    assert_eq!(output, 18);
}

#[test]
fn test_part_2() {
    let parsed_data = day04::read_data_from_string(TEST_DATA_2.to_string());
    let output = day04::find_words_part2(parsed_data);
    assert_eq!(output, 9);
}
