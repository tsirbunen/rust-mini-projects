use calculator::Calculator;
use operator::Operator;
use std::io::stdin;

mod calculator;
mod operator;

fn main() {
    println!("\n\x1b[95m--- {} ---\x1b[0m", "SIMPLE CALCULATOR");
    let first_number = get_number();
    let operator = get_operator();
    let second_number = get_number();

    let calculator = Calculator::new(first_number, second_number, operator.operator_type);
    let result = calculator.calculate();

    println!(
        "\x1b[94mYour calculation:\n{} {} {} = {}\n\x1b[0m",
        first_number, operator.print_value, second_number, result
    );
}

fn get_number() -> f32 {
    let mut input = String::new();
    println!("Enter number:");
    stdin().read_line(&mut input).unwrap();
    let number: f32 = input.trim().parse::<f32>().unwrap();
    number
}

fn get_operator() -> Operator {
    let mut operator = String::new();
    println!("Enter operation ( + , - , * , / ):");
    stdin().read_line(&mut operator).unwrap();
    Operator::new(operator)
}
