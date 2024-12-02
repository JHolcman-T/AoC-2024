pub fn read_data_from_string(input_string: String) -> Vec<Vec<i64>> {
    let parsed_data = input_string
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|entry| entry.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    return parsed_data;
}

pub fn check_allowed_distance(value_1: i64, value_2: i64) -> bool {
    let distance = (value_1 - value_2).abs();
    return 1 <= distance && distance <= 3;
}

pub fn is_valid_record(record: &Vec<i64>) -> (bool, usize) {
    // return boolean and index of failure (0 for positive output)
    let is_decreasing = record[0] > record[1];
    let mut last_entry: Option<&i64> = None;
    for (index, entry) in record.iter().enumerate() {
        match last_entry {
            Some(value) => {
                if !check_allowed_distance(*value, *entry) {
                    return (false, index);
                }
                if (is_decreasing && value <= entry) || (!is_decreasing && value >= entry) {
                    return (false, index);
                }
            }
            None => {}
        }
        last_entry = Some(entry)
    }
    return (true, 0);
}

pub fn is_valid_record_without_element(record: &Vec<i64>, index: usize) -> bool {
    let mut cloned_record = record.clone();
    cloned_record.remove(index);
    return is_valid_record(&cloned_record).0;
}

pub fn categorize_records(records: &Vec<Vec<i64>>) -> Vec<bool> {
    let oputput: Vec<bool> = records
        .iter()
        .map(|record| {
            return is_valid_record(record).0;
        })
        .collect();
    return oputput;
}

pub fn categorize_records_with_errors(records: &Vec<Vec<i64>>) -> Vec<bool> {
    let oputput: Vec<bool> = records
        .iter()
        .map(|record| {
            let record_len = record.len();
            let (is_valid, err_index) = is_valid_record(record);
            if is_valid {
                return true;
            }

            // create 4 copies with removed elements at indicies: err_index, 0, index - 1 and index + 1
            if is_valid_record_without_element(record, err_index) {
                return true;
            }

            // try removing 0
            if err_index != 0 {
                if is_valid_record_without_element(record, 0) {
                    return true;
                }
            }

            // try removing err_index - 1 for indicies > 1
            if err_index > 1 {
                if is_valid_record_without_element(record, err_index - 1) {
                    return true;
                }
            }

            if err_index < (record_len - 1) {
                if is_valid_record_without_element(record, err_index + 1) {
                    return true;
                }
            }

            return false;
        })
        .collect();
    return oputput;
}
