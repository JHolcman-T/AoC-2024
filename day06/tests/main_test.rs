use day06::day06;

const TEST_DATA: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
// const TEST_DATA: &str = ".#..
// .^.#
// #...
// ..#.";

//  0123456789
// 0....#.....
// 1.........#
// 2..........
// 3..#.......
// 4.......#..
// 5..........
// 6.#..^.....
// 7........#.
// 8#.........
// 9......#...

#[cfg(test)]
#[test]
fn test_read_data_from_string() {
    let output = day06::read_data_from_string(TEST_DATA);
    println!("{:?}", output);
    // assert_eq!(output, PARSED_TEST_DATA.to_vec());
}

#[test]
fn test_solve_part_1() {
    let data = day06::read_data_from_string(TEST_DATA);
    let output = day06::solve_part_1(&data);
    println!("{:?}", output);
    assert_eq!(output, 41);
}

#[test]
fn test_solve_part_2() {
    let data = day06::read_data_from_string(TEST_DATA);
    let output = day06::solve_part_2(&data);
    assert_eq!(output, 6);
}
