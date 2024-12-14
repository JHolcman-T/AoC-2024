use day12::day12;

const TEST_DATA: &str = "";

// const PARSED_TEST_DATA;

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = day12::read_data_from_string(TEST_DATA.to_string());
    assert_eq!(output, PARSED_TEST_DATA.to_vec());
}
