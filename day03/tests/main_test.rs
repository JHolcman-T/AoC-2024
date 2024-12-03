use ::day03::day03::Mul;
use day03::day03;

const TEST_DATA: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

const TEST_DATA_2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

const PARSED_TEST_DATA: [[day03::Mul; 4]; 1] = [[
    day03::Mul {
        first_number: 2,
        second_number: 4,
    },
    day03::Mul {
        first_number: 5,
        second_number: 5,
    },
    day03::Mul {
        first_number: 11,
        second_number: 8,
    },
    day03::Mul {
        first_number: 8,
        second_number: 5,
    },
]];

const PARSED_TEST_DATA_2: [[day03::Operation; 6]; 1] = [[
    day03::Operation {
        op_type: day03::OperationType::Mul,
        payload: Some(Mul {
            first_number: 2,
            second_number: 4,
        }),
    },
    day03::Operation {
        op_type: day03::OperationType::OpDont,
        payload: None,
    },
    day03::Operation {
        op_type: day03::OperationType::Mul,
        payload: Some(Mul {
            first_number: 5,
            second_number: 5,
        }),
    },
    day03::Operation {
        op_type: day03::OperationType::Mul,
        payload: Some(Mul {
            first_number: 11,
            second_number: 8,
        }),
    },
    day03::Operation {
        op_type: day03::OperationType::OpDo,
        payload: None,
    },
    day03::Operation {
        op_type: day03::OperationType::Mul,
        payload: Some(Mul {
            first_number: 8,
            second_number: 5,
        }),
    },
]];

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = day03::read_data_from_string(TEST_DATA.to_string());
    assert_eq!(output, PARSED_TEST_DATA);
}

#[test]
fn test_read_data_from_string_extended() {
    let output = day03::read_data_from_string_extended(TEST_DATA_2.to_string());
    assert_eq!(output, PARSED_TEST_DATA_2);
}
