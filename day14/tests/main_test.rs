use day14::day14;

const TEST_DATA: &str = "";

// const PARSED_TEST_DATA;

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = day14::read_data_from_string(TEST_DATA.to_string());
    assert_eq!(output, PARSED_TEST_DATA.to_vec());
}
