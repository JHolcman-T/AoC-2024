pub fn read_data_from_string(input_string: String) -> Vec<Vec<char>> {
    let output = input_string
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    return output;
}

pub fn find_words_part1(input_data: Vec<Vec<char>>) -> u64 {
    let word = "XMAS".chars().collect::<Vec<char>>();
    let word_len = word.len();
    let directions: [(i32, i32); 8] = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let x_max_value = input_data[0].len() - 1;
    let y_max_value = input_data.len() - 1;

    let mut correct_words_count = 0;

    for y_index in 0..=y_max_value {
        for x_index in 0..=x_max_value {
            // iterate over letters

            // exit if char is not 'X'
            if input_data[y_index][x_index] != word[0] {
                continue;
            }

            for (dir_x, dir_y) in directions {
                // initialize values
                let mut word_index = 1;
                let mut current_x = x_index as i32 + dir_x;
                let mut current_y = y_index as i32 + dir_y;

                while word_index < word_len {
                    // out of bounds check
                    if current_x > x_max_value as i32
                        || current_x < 0
                        || current_y > y_max_value as i32
                        || current_y < 0
                    {
                        break;
                    }

                    // char doesn't mach
                    let current_char = input_data[current_y as usize][current_x as usize];
                    let expected_char = word[word_index];
                    if current_char != expected_char {
                        break;
                    }

                    word_index += 1;
                    current_x += dir_x;
                    current_y += dir_y;
                }

                if word_index == word_len {
                    correct_words_count += 1;
                }
            }
        }
    }

    return correct_words_count;
}

pub fn find_words_part2(input_data: Vec<Vec<char>>) -> u64 {
    let word = "MAS".chars().collect::<Vec<char>>();
    let word_len = word.len();

    let x_max_value = input_data[0].len() - 1;
    let y_max_value = input_data.len() - 1;

    let mut correct_words_count = 0;

    for y_index in 0..=y_max_value {
        for x_index in 0..=x_max_value {
            // iterate over letters

            // exit if char is not 'A'
            if input_data[y_index][x_index] != word[1] {
                continue;
            }

            let (x_index_l, x_index_r, x_index_m) =
                (x_index as i32 - 1, x_index as i32 + 1, x_index);

            let (y_index_t, y_index_b, y_index_m) =
                (y_index as i32 - 1, y_index as i32 + 1, y_index);

            if x_index_l < 0
                || x_index_r > x_max_value as i32
                || y_index_t < 0
                || y_index_b > y_max_value as i32
            {
                continue;
            }

            let char_lt = input_data[y_index_t as usize][x_index_l as usize];
            let char_rt = input_data[y_index_t as usize][x_index_r as usize];

            let char_lb = input_data[y_index_b as usize][x_index_l as usize];
            let char_rb = input_data[y_index_b as usize][x_index_r as usize];

            if (char_lt == 'M' && char_rb == 'S' || char_lt == 'S' && char_rb == 'M')
                && (char_rt == 'M' && char_lb == 'S' || char_rt == 'S' && char_lb == 'M')
            {
                correct_words_count += 1;
            }
        }
    }

    return correct_words_count;
}
