#[derive(Clone, Copy)]
enum State {
    ConsumeM,
    ConsumeU,
    ConsumeL,
    ConsumeLeftParen,
    ConsumeRightParen,
    ConsumeFirstNumber,
    ConsumeComma,
    ConsumeSecondNumber,
    ConsumeDoO,
    ConsumeDontSingleQuote,
    ConsumeNorLeftParen,
    ConsumeDontT,
    ConsumeDontLeftParen,
    ConsumeDontRightParen,
    ConsumeDoRightParen,
}

pub fn state_transition(current_state: State) -> State {
    match current_state {
        State::ConsumeM => State::ConsumeU,
        State::ConsumeU => State::ConsumeL,
        State::ConsumeL => State::ConsumeLeftParen,
        State::ConsumeLeftParen => State::ConsumeFirstNumber,
        State::ConsumeFirstNumber => State::ConsumeComma,
        State::ConsumeComma => State::ConsumeSecondNumber,
        State::ConsumeSecondNumber => State::ConsumeRightParen,
        State::ConsumeRightParen => State::ConsumeM,
        _ => State::ConsumeM,
    }
}

#[derive(Debug)]
pub struct Mul {
    pub first_number: i64,
    pub second_number: i64,
}

impl PartialEq for Mul {
    fn eq(&self, other: &Self) -> bool {
        return self.first_number == other.first_number
            && self.second_number == other.second_number;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OperationType {
    OpDo,
    OpDont,
    Mul,
}

#[derive(Debug)]
pub struct Operation {
    pub op_type: OperationType,
    pub payload: Option<Mul>,
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        return self.op_type == other.op_type && self.payload == other.payload;
    }
}

pub fn read_data_from_string(input_string: String) -> Vec<Vec<Mul>> {
    // "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let parsed_data = input_string
        .lines()
        .map(|line| {
            let mut operations: Vec<Mul> = vec![];
            let mut first_number: i64 = 0;
            let mut second_number: i64 = 0;
            let mut current_state: State = State::ConsumeM;
            let mut tmp_number = String::new();
            for char in line.chars() {
                match (char, current_state) {
                    ('m', State::ConsumeM) => current_state = state_transition(current_state),
                    ('u', State::ConsumeU) => current_state = state_transition(current_state),
                    ('l', State::ConsumeL) => current_state = state_transition(current_state),
                    ('(', State::ConsumeLeftParen) => {
                        current_state = state_transition(current_state);
                    }
                    (',', State::ConsumeFirstNumber) => {
                        current_state = state_transition(State::ConsumeComma);

                        if tmp_number.len() > 0 {
                            first_number = tmp_number.parse::<i64>().unwrap();
                            tmp_number = String::new();
                        }
                    }
                    (')', State::ConsumeSecondNumber) => {
                        current_state = state_transition(State::ConsumeRightParen);

                        if tmp_number.len() > 0 {
                            second_number = tmp_number.parse::<i64>().unwrap();
                            tmp_number = String::new();
                        }

                        // save operation
                        operations.push(Mul {
                            first_number: first_number.clone(),
                            second_number: second_number.clone(),
                        });

                        // reset the state helpers
                        first_number = 0;
                        second_number = 0;
                        tmp_number = String::new();
                    }
                    ('0', State::ConsumeFirstNumber)
                    | ('1', State::ConsumeFirstNumber)
                    | ('2', State::ConsumeFirstNumber)
                    | ('3', State::ConsumeFirstNumber)
                    | ('4', State::ConsumeFirstNumber)
                    | ('5', State::ConsumeFirstNumber)
                    | ('6', State::ConsumeFirstNumber)
                    | ('7', State::ConsumeFirstNumber)
                    | ('8', State::ConsumeFirstNumber)
                    | ('9', State::ConsumeFirstNumber) => tmp_number += char.to_string().as_str(),
                    ('0', State::ConsumeSecondNumber)
                    | ('1', State::ConsumeSecondNumber)
                    | ('2', State::ConsumeSecondNumber)
                    | ('3', State::ConsumeSecondNumber)
                    | ('4', State::ConsumeSecondNumber)
                    | ('5', State::ConsumeSecondNumber)
                    | ('6', State::ConsumeSecondNumber)
                    | ('7', State::ConsumeSecondNumber)
                    | ('8', State::ConsumeSecondNumber)
                    | ('9', State::ConsumeSecondNumber) => tmp_number += char.to_string().as_str(),
                    _ => {
                        // reset the state helpers
                        current_state = State::ConsumeM;
                        first_number = 0;
                        second_number = 0;
                        tmp_number = String::new();
                    }
                }
            }
            return operations;
        })
        .collect::<Vec<Vec<Mul>>>();

    return parsed_data;
}

pub fn read_data_from_string_extended(input_string: String) -> Vec<Vec<Operation>> {
    // "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    let parsed_data = input_string
        .lines()
        .map(|line| {
            let mut operations: Vec<Operation> = vec![];
            let mut first_number: i64 = 0;
            let mut second_number: i64 = 0;
            let mut current_state: State = State::ConsumeM;
            let mut tmp_number = String::new();
            for char in line.chars() {
                match (char, current_state) {
                    ('m', _) => current_state = state_transition(State::ConsumeM),
                    ('u', State::ConsumeU) => current_state = state_transition(current_state),
                    ('l', State::ConsumeL) => current_state = state_transition(current_state),
                    ('(', State::ConsumeLeftParen) => {
                        current_state = state_transition(current_state);
                    }
                    (',', State::ConsumeFirstNumber) => {
                        current_state = state_transition(State::ConsumeComma);

                        if tmp_number.len() > 0 {
                            first_number = tmp_number.parse::<i64>().unwrap();
                            tmp_number = String::new();
                        }
                    }
                    (')', State::ConsumeSecondNumber) => {
                        current_state = state_transition(State::ConsumeRightParen);

                        if tmp_number.len() > 0 {
                            second_number = tmp_number.parse::<i64>().unwrap();
                            tmp_number = String::new();
                        }

                        // save operation
                        operations.push(Operation {
                            op_type: OperationType::Mul,
                            payload: Some(Mul {
                                first_number: first_number.clone(),
                                second_number: second_number.clone(),
                            }),
                        });

                        // reset the state helpers
                        first_number = 0;
                        second_number = 0;
                        tmp_number = String::new();
                    }
                    ('0', State::ConsumeFirstNumber)
                    | ('1', State::ConsumeFirstNumber)
                    | ('2', State::ConsumeFirstNumber)
                    | ('3', State::ConsumeFirstNumber)
                    | ('4', State::ConsumeFirstNumber)
                    | ('5', State::ConsumeFirstNumber)
                    | ('6', State::ConsumeFirstNumber)
                    | ('7', State::ConsumeFirstNumber)
                    | ('8', State::ConsumeFirstNumber)
                    | ('9', State::ConsumeFirstNumber) => tmp_number += char.to_string().as_str(),
                    ('0', State::ConsumeSecondNumber)
                    | ('1', State::ConsumeSecondNumber)
                    | ('2', State::ConsumeSecondNumber)
                    | ('3', State::ConsumeSecondNumber)
                    | ('4', State::ConsumeSecondNumber)
                    | ('5', State::ConsumeSecondNumber)
                    | ('6', State::ConsumeSecondNumber)
                    | ('7', State::ConsumeSecondNumber)
                    | ('8', State::ConsumeSecondNumber)
                    | ('9', State::ConsumeSecondNumber) => tmp_number += char.to_string().as_str(),
                    // do
                    ('d', _) => current_state = State::ConsumeDoO,
                    ('o', State::ConsumeDoO) => current_state = State::ConsumeNorLeftParen,
                    ('n', State::ConsumeNorLeftParen) => {
                        current_state = State::ConsumeDontSingleQuote
                    }
                    ('\'', State::ConsumeDontSingleQuote) => current_state = State::ConsumeDontT,
                    ('t', State::ConsumeDontT) => current_state = State::ConsumeDontLeftParen,
                    ('(', State::ConsumeDontLeftParen) => {
                        current_state = State::ConsumeDontRightParen
                    }
                    (')', State::ConsumeDontRightParen) => {
                        operations.push(Operation {
                            op_type: OperationType::OpDont,
                            payload: None,
                        });
                    }
                    ('(', State::ConsumeNorLeftParen) => current_state = State::ConsumeDoRightParen,
                    (')', State::ConsumeDoRightParen) => {
                        operations.push(Operation {
                            op_type: OperationType::OpDo,
                            payload: None,
                        });
                    }
                    _ => {
                        // reset the state helpers
                        current_state = State::ConsumeM;
                        first_number = 0;
                        second_number = 0;
                        tmp_number = String::new();
                    }
                }
            }
            return operations;
        })
        .collect::<Vec<Vec<Operation>>>();

    return parsed_data;
}
