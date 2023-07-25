use std::io;
#[derive(Debug)]
#[allow(unused_variables)]

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn calculate(operation: &Operation) -> f64 {

    match operation {
        Operation::Add(data1, data2) => data1 + data2,
        Operation::Subtract(data1, data2) => data1 - data2,
        Operation::Multiply(data1, data2) => data1 * data2,
        Operation::Divide(data1, data2) => data1 / data2,
    }
}

fn main() {

    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let first_number: f64 = input.trim().parse().expect("Invalid input");

    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation = match input.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    println!("Enter the second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let second_number: f64 = input.trim().parse().expect("Invalid input");

    let operation_instance = operation(first_number, second_number);
    let result = calculate(&operation_instance);

    println!("Result: {}", result);
}

