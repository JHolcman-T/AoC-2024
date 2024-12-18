use dayXX::dayXX;

const TEST_DATA: &str = "";

// const PARSED_TEST_DATA;

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = dayXX::read_data_from_string(TEST_DATA.to_string());
    assert_eq!(output, PARSED_TEST_DATA.to_vec());
}
