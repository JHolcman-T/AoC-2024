pub fn read_data_from_string(input_string: String) -> Vec<(i64, Vec<i64>)> {
    input_string
        .lines()
        .map(|line| {
            let parts = line.trim().split(":").collect::<Vec<&str>>();
            let result = parts[0].parse::<i64>().unwrap();
            let numbers = parts[1]
                .split_whitespace()
                .map(|number| number.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            return (result, numbers);
        })
        .collect::<Vec<(i64, Vec<i64>)>>()
}

pub fn is_valid_calculation_part_1(expected_result: i64, numbers: Vec<i64>) -> bool {
    let last_index = numbers.len() - 1;
    let last_item = numbers[last_index];

    // check if reduced expected_result matches the last remaining item
    if numbers.len() == 1 {
        return expected_result == numbers[0];
    }

    let reduced_numbers = numbers[..last_index].to_vec();

    // try by subtraction (inverse '+')
    if is_valid_calculation_part_1(expected_result - last_item, reduced_numbers.clone()) {
        return true;
    }
    // try by divide (inverse '*')
    if expected_result % last_item == 0
        && is_valid_calculation_part_1(expected_result / last_item, reduced_numbers.clone())
    {
        return true;
    }
    return false;
}

pub fn is_valid_calculation_part_2(expected_result: i64, numbers: Vec<i64>) -> bool {
    // check if reduced expected_result matches the last remaining item
    if numbers.len() == 1 {
        return expected_result == numbers[0];
    }

    // try by adding
    if is_valid_calculation_part_2(
        expected_result,
        [&[numbers[0] + numbers[1]], &numbers[2..]].concat(),
    ) {
        return true;
    }
    // try by multiply
    if is_valid_calculation_part_2(
        expected_result,
        [&[numbers[0] * numbers[1]], &numbers[2..]].concat(),
    ) {
        return true;
    }
    // try by concat
    if is_valid_calculation_part_2(
        expected_result,
        [
            &[(numbers[0].to_string() + &numbers[1].to_string())
                .parse::<i64>()
                .unwrap()],
            &numbers[2..],
        ]
        .concat(),
    ) {
        return true;
    }
    return false;
}

pub fn solve_part_1(data: &Vec<(i64, Vec<i64>)>) -> i64 {
    let output = data
        .iter()
        .filter(|entry| is_valid_calculation_part_1(entry.0, entry.1.clone()))
        .map(|entry| entry.0)
        .sum();
    return output;
}

pub fn solve_part_2(data: &Vec<(i64, Vec<i64>)>) -> i64 {
    let output = data
        .iter()
        .filter(|entry| is_valid_calculation_part_2(entry.0, entry.1.clone()))
        .map(|entry| entry.0)
        .sum();
    return output;
}
