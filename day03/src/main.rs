use ::day03::day03::Mul;
use ::day03::day03::Operation;
use ::day03::day03::OperationType;
use day03::day03;

const DATA: &str = include_str!("../data/data.txt");

fn part1(parsed_data: Vec<Vec<Mul>>) {
    let output: i64 = parsed_data
        .iter()
        .map(|line| {
            line.iter()
                .map(|oper| oper.first_number * oper.second_number)
                .sum::<i64>()
        })
        .sum();
    println!("Part 1: {output}");
}

fn part2(parsed_data: Vec<Vec<Operation>>) {
    let flatten_data = parsed_data.iter().flatten().collect::<Vec<&Operation>>();
    let mut output_vec: Vec<&Mul> = vec![];
    let mut skip = false;
    for operation in flatten_data {
        match (operation.op_type, skip) {
            (OperationType::OpDont, _) => skip = true,
            (OperationType::OpDo, _) => skip = false,
            (OperationType::Mul, false) => output_vec.push(operation.payload.as_ref().unwrap()),
            (OperationType::Mul, true) => continue,
        }
    }
    let output = output_vec
        .iter()
        .map(|oper| oper.first_number * oper.second_number)
        .sum::<i64>();
    println!("Part 2: {output}");
}

fn main() {
    let parsed_data = day03::read_data_from_string(DATA.to_string());
    let parsed_data_2 = day03::read_data_from_string_extended(DATA.to_string());
    part1(parsed_data);
    part2(parsed_data_2);
}
