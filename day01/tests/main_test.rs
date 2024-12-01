use day01::day01;

const LIST_LEFT: [i64; 6] = [3, 4, 2, 1, 3, 3];
const LIST_RIGHT: [i64; 6] = [4, 3, 5, 3, 9, 3];

#[cfg(test)]
#[test]
fn test_sum_lists() {
    assert_eq!(day01::sum_lists(&LIST_LEFT, &LIST_RIGHT), 11);
}
#[test]
fn test_read_from_string() {
    let data_string = "3   4
4   3
2   5
1   3
3   9
3   3";
    let (list_left, list_right) = day01::read_data_from_string(data_string.to_string());
    assert_eq!(LIST_LEFT, list_left.as_slice());
    assert_eq!(LIST_RIGHT, list_right.as_slice());
}

#[test]
fn test_count_and_sum() {
    let sum = day01::count_and_sum(&LIST_LEFT, &LIST_RIGHT);
    assert_eq!(sum, 31);
}
