use std::env::{args, Args};

fn main() {
    // println!("Hello, world!");
    let mut args: Args = args();
    let first: String = args.nth(1).unwrap();
    let operator: String = args.nth(0).unwrap();
    let second: String = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let operator_char = operator.chars().next().unwrap();

    let result = operate(operator_char, first_number, second_number);
    println!("{} {} {} = {}", first_number, operator_char, second_number, result);
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '%' => first_number % second_number,
        '/' => first_number / second_number,
        '*' | 'X' | 'x' => first_number * second_number,
        _ => panic!("Invalid operator: {}", operator),
    }
}
