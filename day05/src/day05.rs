use std::collections::{hash_map::RandomState, HashSet};

#[derive(Debug)]
pub struct Rule {
    pub first_page: usize,
    pub second_page: usize,
}

#[derive(Debug)]
pub struct ParsedData {
    pub rules: HashSet<(usize, usize)>,
    pub update_orders: Vec<Vec<usize>>,
}

pub fn read_data_from_string(input_string: String) -> ParsedData {
    let data: Vec<&str> = input_string.lines().collect();
    let separation_index = data.iter().position(|line| *line == "").unwrap();
    let rules = &data[..separation_index];
    let update_orders = &data[separation_index + 1..];

    let parsed_rules: HashSet<(usize, usize)> = rules
        .iter()
        .map(|rule| {
            let values: Vec<&str> = rule.split("|").collect();
            assert_eq!(values.len(), 2);
            return {
                (
                    values[0].parse::<usize>().unwrap(),
                    values[1].parse::<usize>().unwrap(),
                )
            };
        })
        .collect();

    let parsed_update_orders: Vec<Vec<usize>> = update_orders
        .iter()
        .map(|update_order| {
            update_order
                .split(",")
                .map(|value| value.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    let parsed_data = ParsedData {
        rules: parsed_rules,
        update_orders: parsed_update_orders,
    };

    return parsed_data;
}

pub fn is_valid_update_order(rules: &HashSet<(usize, usize)>, update_order: &Vec<usize>) -> bool {
    let max_len = update_order.len() - 1;
    let mut index: usize = 0;
    while index < max_len {
        let window: (usize, usize) = (update_order[index], update_order[index + 1]);
        if !rules.contains(&window) {
            return false;
        }
        index += 1;
    }
    return true;
}

pub fn fix_order(rules: &HashSet<(usize, usize)>, update_order: &Vec<usize>) -> Vec<usize> {
    let mut corrected_update_order: Vec<usize> = update_order.clone();
    let max_len = corrected_update_order.len() - 1;

    while !is_valid_update_order(rules, &corrected_update_order) {
        let mut index: usize = 0;
        while index < max_len {
            let window: (usize, usize) = (
                corrected_update_order[index],
                corrected_update_order[index + 1],
            );
            if !rules.contains(&window) {
                if rules.contains(&(window.1, window.0)) {
                    corrected_update_order[index] = window.1;
                    corrected_update_order[index + 1] = window.0;
                } else {
                    panic!("Unknown state!");
                }
            }
            index += 1;
        }
    }

    return corrected_update_order;
}

pub fn get_incorrect_ordered_updates(parsed_data: &ParsedData) -> Vec<Vec<usize>> {
    let invalid_update_orders = parsed_data
        .update_orders
        .iter()
        .filter(|update_order| !is_valid_update_order(&parsed_data.rules, update_order))
        .cloned()
        .collect::<Vec<Vec<usize>>>();
    return invalid_update_orders;
}

pub fn calculate_valid_middle_numbers_sum(parsed_data: &ParsedData) -> usize {
    let valid_middle_numbers = parsed_data
        .update_orders
        .iter()
        .map(|update_order| {
            let middle_index: usize = (update_order.len() - 1) / 2;
            if is_valid_update_order(&parsed_data.rules, update_order) {
                return update_order[middle_index];
            }
            return 0;
        })
        .sum();
    return valid_middle_numbers;
}

pub fn calculate_middle_numbers_sum_from_corrected_update_orders(
    parsed_data: &ParsedData,
) -> usize {
    let sum: usize = get_incorrect_ordered_updates(parsed_data)
        .iter()
        .map(|incorrect_update_order| {
            let fixed_update_order = fix_order(&parsed_data.rules, incorrect_update_order);
            let middle_index = (fixed_update_order.len() - 1) / 2;
            return fixed_update_order[middle_index];
        })
        .sum();
    return sum;
}
