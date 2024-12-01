use std::{collections::HashMap, iter::zip};

pub fn sum_lists(list_left: &[i64], list_right: &[i64]) -> i64 {
    // ensure equal length
    assert_eq!(list_left.len(), list_right.len());

    let mut list_left = list_left.to_vec();
    let mut list_right = list_right.to_vec();

    list_left.sort();
    list_right.sort();

    let combined = zip(list_left, list_right);

    let result: i64 = combined.map(|entry| (entry.0 - entry.1).abs()).sum();

    return result;
}

pub fn count_and_sum(list_left: &[i64], list_right: &[i64]) -> i64 {
    // ensure equal length
    assert_eq!(list_left.len(), list_right.len());

    let mut list_left = list_left.to_vec();
    let mut list_right = list_right.to_vec();

    list_left.sort();
    list_right.sort();

    // create map of frequencies in right list
    let mut count_map: HashMap<i64, i64> = HashMap::new();
    for number in list_right {
        match count_map.get(&number) {
            Some(count) => count_map.insert(number, count + 1),
            None => count_map.insert(number, 1),
        };
    }

    // calculate sum based on the map create before
    let sum = list_left
        .iter()
        .map(|entry| match count_map.get(entry) {
            Some(value) => {
                return value * entry;
            }
            None => {
                return 0;
            }
        })
        .sum();

    return sum;
}

pub fn read_data_from_string(string: String) -> (Vec<i64>, Vec<i64>) {
    let lines = string.lines();
    let zipped_lists = lines.map(|line| {
        let mut splitted = line.split_whitespace();
        assert_eq!(splitted.clone().count(), 2);
        let tuple = (
            splitted.next().unwrap().parse::<i64>().unwrap(),
            splitted.next().unwrap().parse::<i64>().unwrap(),
        );
        return tuple;
    });

    let (list_left, list_right): (Vec<i64>, Vec<i64>) = zipped_lists.unzip();

    return (list_left, list_right);
}
