use super::operator::OperatorType;

pub struct Calculator {
    first_number: f32,
    second_number: f32,
    operator_type: OperatorType,
}

impl Calculator {
    pub fn new(first_number: f32, second_number: f32, operator_type: OperatorType) -> Self {
        Self {
            first_number,
            second_number,
            operator_type,
        }
    }

    pub fn calculate(self) -> f32 {
        match self.operator_type {
            OperatorType::Add => self.first_number + self.second_number,
            OperatorType::Subtract => self.first_number - self.second_number,
            OperatorType::Multiply => self.first_number * self.second_number,
            OperatorType::Divide => {
                if self.second_number == 0.0 {
                    panic!("Cannot divide by zero!");
                }

                self.first_number / self.second_number
            }
        }
    }
}

#[cfg(test)]
mod calculator_tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(1.0, 1.0, 2.0)]
    #[case(1.0, 0.0, 1.0)]
    #[case(0.3, 1.9, 2.2)]
    #[case(12.0, 13.0, 25.0)]
    fn adding_two_numbers_works(#[case] number1: f32, #[case] number2: f32, #[case] expected: f32) {
        let calculator = Calculator::new(number1, number2, OperatorType::Add);
        assert_eq!(calculator.calculate(), expected);
    }
}
