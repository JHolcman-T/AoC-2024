use std::collections::HashSet;

use day08::day08;

const TEST_DATA: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
// const TEST_DATA: &str = "..........
// ..........
// ..........
// ....a.....
// ..........
// .....a....
// ..........
// ..........
// ..........
// ..........";

// const PARSED_TEST_DATA;

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    use std::collections::HashMap;

    let (output, _) = day08::read_data_from_string(&TEST_DATA.to_string());
    let expected: HashMap<char, Vec<day08::Point>> = HashMap::from([
        (
            '0',
            vec![
                day08::Point { x: 8, y: 1 },
                day08::Point { x: 5, y: 2 },
                day08::Point { x: 7, y: 3 },
                day08::Point { x: 4, y: 4 },
            ],
        ),
        (
            'A',
            vec![
                day08::Point { x: 6, y: 5 },
                day08::Point { x: 8, y: 8 },
                day08::Point { x: 9, y: 9 },
            ],
        ),
    ]);
    assert_eq!(output, expected);
}

#[test]
fn test_linear_function_from_points() {
    let point1 = day08::Point { x: 1, y: 1 };
    let point2 = day08::Point { x: 2, y: 2 };

    let out_f = day08::linear_function_from_points(&point1, &point2);
    let output = out_f(3 as f64);
    assert_eq!(output, 3);
}

#[test]
fn test_get_antinodes_part_1() {
    let (antennas, (map_w, map_h)) = day08::read_data_from_string(&TEST_DATA.to_string());
    let output = day08::get_antinodes_part_1(&antennas, map_w, map_h);
    let expected = HashSet::from([
        day08::Point { x: 1, y: 5 },
        day08::Point { x: 3, y: 6 },
        day08::Point { x: 3, y: 1 },
        day08::Point { x: 6, y: 0 },
        day08::Point { x: 10, y: 2 },
        day08::Point { x: 10, y: 10 },
        day08::Point { x: 0, y: 7 },
        day08::Point { x: 11, y: 0 },
        day08::Point { x: 7, y: 7 },
        day08::Point { x: 10, y: 11 },
        day08::Point { x: 6, y: 5 },
        day08::Point { x: 4, y: 2 },
        day08::Point { x: 2, y: 3 },
        day08::Point { x: 9, y: 4 },
    ]);
    assert_eq!(output, expected);
}

#[test]
fn test_solve_part_1() {
    let (antennas, (map_w, map_h)) = day08::read_data_from_string(&TEST_DATA.to_string());
    let output = day08::solve_part_1(&antennas, map_w, map_h);
    assert_eq!(output, 14);
}

#[test]
fn test_get_antinodes_part_2() {
    let (antennas, (map_w, map_h)) = day08::read_data_from_string(&TEST_DATA.to_string());
    let output = day08::get_antinodes_part_2(&antennas, map_w, map_h);
    let expected = HashSet::from([
        day08::Point { x: 10, y: 11 },
        day08::Point { x: 3, y: 3 },
        day08::Point { x: 4, y: 2 },
        day08::Point { x: 3, y: 11 },
        day08::Point { x: 9, y: 4 },
        day08::Point { x: 4, y: 9 },
        day08::Point { x: 5, y: 5 },
        day08::Point { x: 11, y: 5 },
        day08::Point { x: 0, y: 7 },
        day08::Point { x: 5, y: 7 },
        day08::Point { x: 10, y: 10 },
        day08::Point { x: 1, y: 10 },
        day08::Point { x: 3, y: 6 },
        day08::Point { x: 9, y: 9 },
        day08::Point { x: 10, y: 2 },
        day08::Point { x: 7, y: 3 },
        day08::Point { x: 4, y: 4 },
        day08::Point { x: 8, y: 8 },
        day08::Point { x: 6, y: 5 },
        day08::Point { x: 2, y: 8 },
        day08::Point { x: 6, y: 0 },
        day08::Point { x: 11, y: 11 },
        day08::Point { x: 5, y: 2 },
        day08::Point { x: 1, y: 1 },
        day08::Point { x: 1, y: 0 },
        day08::Point { x: 7, y: 7 },
        day08::Point { x: 2, y: 3 },
        day08::Point { x: 8, y: 1 },
        day08::Point { x: 3, y: 1 },
        day08::Point { x: 0, y: 0 },
        day08::Point { x: 6, y: 6 },
        day08::Point { x: 2, y: 2 },
        day08::Point { x: 11, y: 0 },
        day08::Point { x: 1, y: 5 },
    ]);
    assert_eq!(output, expected);
}

#[test]
fn test_solve_part_2() {
    let (antennas, (map_w, map_h)) = day08::read_data_from_string(&TEST_DATA.to_string());
    let output = day08::solve_part_2(&antennas, map_w, map_h);
    assert_eq!(output, 34);
}
