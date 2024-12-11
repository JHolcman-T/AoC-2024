use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

pub fn read_data_from_string(input_string: &String) -> (HashMap<char, Vec<Point>>, (i64, i64)) {
    let mut collection: HashMap<char, Vec<Point>> = HashMap::new();
    let data: Vec<&str> = input_string.lines().collect();
    let map_h = data.len() as i64;
    let map_w = data[0].len() as i64;
    for (row_index, row_data) in input_string.lines().enumerate() {
        for (column_index, column_data) in row_data.char_indices() {
            if column_data == '.' {
                continue;
            }
            let point = Point {
                x: column_index as i64,
                y: row_index as i64,
            };
            collection
                .entry(column_data)
                .and_modify(|value| value.push(point))
                .or_insert(vec![point]);
        }
    }
    return (collection, (map_w, map_h));
}

pub fn linear_function_from_points(point1: &Point, point2: &Point) -> impl Fn(f64) -> i64 {
    assert_ne!(point1.x, point2.x, "Points cannot have the same X value!");
    let greater_point = if point1.x > point2.x { point1 } else { point2 };
    let smaller_point = if point1.x < point2.x { point1 } else { point2 };

    let m_t = (greater_point.y - smaller_point.y) as f64;
    let m_b = (greater_point.x - smaller_point.x) as f64;
    let m: f64 = m_t / m_b;
    let t = greater_point.y as f64 - (greater_point.x as f64 * m);

    let f = move |x: f64| {
        (x * m + t)
            .max(std::i64::MIN as f64)
            .min(std::i64::MAX as f64)
            .round() as i64
    };
    return f;
}

pub fn get_antinodes_part_1(
    antennas: &HashMap<char, Vec<Point>>,
    map_w: i64,
    map_h: i64,
) -> HashSet<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (_, points) in antennas {
        for (point1, point2) in points.iter().tuple_combinations::<(_, _)>() {
            let greater_point = if point1.x > point2.x { point1 } else { point2 };
            let smaller_point = if point1.x < point2.x { point1 } else { point2 };

            let out_f = linear_function_from_points(point1, point2);
            let x_axis_distance = (point1.x - point2.x).abs();

            let antinode_1_x = smaller_point.x - x_axis_distance;
            let antinode_1 = Point {
                x: antinode_1_x,
                y: out_f(antinode_1_x as f64),
            };

            let antinode_2_x = greater_point.x + x_axis_distance;
            let antinode_2 = Point {
                x: antinode_2_x,
                y: out_f(antinode_2_x as f64),
            };

            for antinode in [antinode_1, antinode_2] {
                if antinode.x < 0 || antinode.y < 0 || antinode.x >= map_w || antinode.y >= map_h {
                    continue;
                }
                antinodes.insert(antinode);
            }
        }
    }
    return antinodes;
}

pub fn solve_part_1(antennas: &HashMap<char, Vec<Point>>, map_w: i64, map_h: i64) -> usize {
    let antinodes = get_antinodes_part_1(antennas, map_w, map_h);

    // debug print antinodes on the map
    // let mut out_map = vec![vec!['.'; map_w as usize]; map_h as usize];

    // for antinode in &antinodes {
    //     out_map[antinode.y as usize][antinode.x as usize] = '#';
    // }

    // let out_map_str = out_map
    //     .iter()
    //     .map(|line| line.iter().collect::<String>())
    //     .join("\n");

    // println!("{}", out_map_str);

    return antinodes.len();
}

pub fn get_antinodes_part_2(
    antennas: &HashMap<char, Vec<Point>>,
    map_w: i64,
    map_h: i64,
) -> HashSet<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();
    for (_, points) in antennas {
        for (point1, point2) in points.iter().tuple_combinations::<(_, _)>() {
            let greater_point = if point1.x > point2.x { point1 } else { point2 };
            let smaller_point = if point1.x < point2.x { point1 } else { point2 };

            let out_f = linear_function_from_points(point1, point2);
            let x_axis_distance = (point1.x - point2.x).abs();

            // negative
            let mut current_point = *smaller_point;
            loop {
                let antinode_x = current_point.x - x_axis_distance;

                // too small
                if antinode_x < 0 {
                    break;
                }

                let antinode = Point {
                    x: antinode_x,
                    y: out_f(antinode_x as f64),
                };

                current_point = antinode;

                if antinode.x < 0 || antinode.y < 0 || antinode.x >= map_w || antinode.y >= map_h {
                    continue;
                }

                antinodes.insert(antinode);
            }

            // positive
            current_point = *greater_point;
            loop {
                let antinode_x = current_point.x + x_axis_distance;

                // too large
                if antinode_x >= map_w {
                    break;
                }

                let antinode = Point {
                    x: antinode_x,
                    y: out_f(antinode_x as f64),
                };

                current_point = antinode;

                if antinode.x < 0 || antinode.y < 0 || antinode.x >= map_w || antinode.y >= map_h {
                    continue;
                }

                antinodes.insert(antinode);
            }

            // antinodes at antenna points
            antinodes.insert(*smaller_point);
            antinodes.insert(*greater_point);
        }
    }
    return antinodes;
}

pub fn solve_part_2(antennas: &HashMap<char, Vec<Point>>, map_w: i64, map_h: i64) -> usize {
    let antinodes = get_antinodes_part_2(antennas, map_w, map_h);

    // debug print antinodes on the map
    // let mut out_map = vec![vec!['.'; map_w as usize]; map_h as usize];

    // for antinode in &antinodes {
    //     out_map[antinode.y as usize][antinode.x as usize] = '#';
    // }

    // let out_map_str = out_map
    //     .iter()
    //     .map(|line| line.iter().collect::<String>())
    //     .join("\n");

    // println!("{}", out_map_str);

    return antinodes.len();
}
