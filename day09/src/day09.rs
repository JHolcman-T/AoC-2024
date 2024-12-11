pub fn read_data_from_string(input_string: String) -> Vec<String> {
    input_string
        .char_indices()
        .flat_map(|(index, number)| {
            let parsed_number: usize = number.to_digit(10).unwrap() as usize;
            if index % 2 == 0 {
                let file_no = index / 2;
                return vec![file_no.to_string(); parsed_number];
            } else {
                return vec![".".to_string(); parsed_number];
            }
        })
        .collect::<Vec<String>>()
}

pub fn solve_part_1(data: &Vec<String>) -> usize {
    let mut input_data = data.clone();
    let len = input_data.len();

    let mut start = 0;
    for index in (0..len).rev() {
        let file_item = input_data[index].clone();
        if file_item == "." {
            continue;
        }
        for free_space_index in start..len {
            if free_space_index > index {
                break;
            };
            let free_space = &input_data[free_space_index];
            if free_space != "." {
                continue;
            }
            input_data[free_space_index] = file_item.clone();
            input_data[index] = ".".to_string();
            start = free_space_index;
            break;
        }
    }

    let mut result: usize = 0;

    for (index, element) in input_data.iter().enumerate() {
        if element == "." {
            continue;
        }
        result += index * element.parse::<usize>().unwrap();
    }
    return result;
}

pub fn solve_part_2(data: &Vec<String>) -> usize {
    let mut input_data = data.clone();
    let len = input_data.len();

    // group files to blocks

    let mut grouped_blocks: Vec<(&str, usize, usize)> = vec![];
    let mut current_processed = "0";
    let mut elements_seen: usize = 0;
    let mut begin_index: usize = 0;

    // let binding = input_data.clone();
    let mut binding = input_data.clone();
    // add element for convinience
    binding.push(".".to_string());
    for (index, element) in binding.iter().enumerate() {
        if element != current_processed {
            if current_processed != "." {
                grouped_blocks.push((current_processed, begin_index, elements_seen));
            }
            begin_index = index;
            current_processed = element;
            elements_seen = 1;
            continue;
        }
        elements_seen += 1;
    }

    // move files on disk

    for (file_id, file_block_begin, file_block_len) in grouped_blocks.iter().rev() {
        let mut free_space_window_size: usize = 0;
        for free_space_index in 0..len {
            if free_space_index > *file_block_begin {
                break;
            }
            let free_space_element = input_data[free_space_index].clone();
            if free_space_element == "." {
                free_space_window_size += 1;
            }
            if free_space_window_size != 0 && free_space_element != "." {
                if free_space_window_size < *file_block_len {
                    // reset -> file doesn't fit the free space block
                    free_space_window_size = 0;
                    continue;
                }

                for file_chunk in 0..*file_block_len {
                    input_data[file_block_begin + file_chunk] = ".".to_string();
                    input_data[(free_space_index - free_space_window_size) + file_chunk] =
                        file_id.to_string();
                }
                break;
            }
        }
    }

    let mut result: usize = 0;

    for (index, element) in input_data.iter().enumerate() {
        if element == "." {
            continue;
        }
        result += index * element.parse::<usize>().unwrap();
    }

    return result;
}
