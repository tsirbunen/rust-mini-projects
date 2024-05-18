#[derive(Debug, Copy, Clone)]
pub enum OperatorType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub struct Operator {
    pub operator_type: OperatorType,
    pub print_value: String,
}

impl Operator {
    pub fn new(input_value: String) -> Self {
        let value = input_value.trim();

        Self {
            operator_type: get_operator_type(value),
            print_value: value.to_string(),
        }
    }
}

fn get_operator_type(operator: &str) -> OperatorType {
    match operator {
        "+" => OperatorType::Add,
        "-" => OperatorType::Subtract,
        "*" => OperatorType::Multiply,
        "/" => OperatorType::Divide,
        _ => panic!("Invalid operator!"),
    }
}
