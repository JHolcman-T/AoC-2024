use std::collections::HashMap;

pub fn read_data_from_string(input_string: String) -> Vec<i64> {
    input_string
        .split_whitespace()
        .map(|el| el.parse::<i64>().unwrap())
        .collect()
}

pub fn solve(data: &Vec<i64>, steps: i64) -> i64 {
    let mut cache: HashMap<(i64, i64), i64> = HashMap::new();
    fn get_len(number: i64, step: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
        let stone = number.to_string();
        let stone_len = stone.len();
        if cache.contains_key(&(number, step)) {
            let value = cache.get(&(number, step)).unwrap();
            return *value;
        }
        let ret: i64;
        if step == 0 {
            ret = 1;
        } else if number == 0 {
            ret = get_len(1, step - 1, cache)
        } else if stone_len % 2 == 0 {
            let middle = stone_len / 2;
            let left_stone = stone[..middle].parse::<i64>().unwrap();
            let right_stone = stone[middle..].parse::<i64>().unwrap();
            ret = get_len(left_stone, step - 1, cache) + get_len(right_stone, step - 1, cache);
        } else {
            ret = get_len(number * 2024, step - 1, cache);
        }
        cache.insert((number, step), ret);
        return ret;
    }
    return data
        .iter()
        .map(|number| get_len(*number, steps, &mut cache))
        .sum();
}

pub fn solve_part1(data: &Vec<i64>) -> i64 {
    return solve(data, 25);
}

pub fn solve_part2(data: &Vec<i64>) -> i64 {
    return solve(data, 75);
}
