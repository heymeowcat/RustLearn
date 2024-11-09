use std::io;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn parse_operation(input: &str) -> Option<Operation> {
    match input.to_lowercase().as_str() {
        "add" => Some(Operation::Add),
        "subtract" => Some(Operation::Subtract),
        "multiply" => Some(Operation::Multiply),
        "divide" => Some(Operation::Divide),
        _ => None,
    }
}

fn calculate(op: Operation, x: f64, y: f64) -> Result<f64, String> {
    match op {
        Operation::Add => Ok(x + y),
        Operation::Subtract => Ok(x - y),
        Operation::Multiply => Ok(x * y),
        Operation::Divide => {
            if y == 0.0 {
                Err("Cannot divide by zero.".to_string())
            } else {
                Ok(x / y)
            }
        }
    }
}

fn main() {
    println!("Enter operation (add, subtract, multiply, divide):");

    let mut operation_input = String::new();
    io::stdin()
        .read_line(&mut operation_input)
        .expect("Failed to read line");
    let operation_input = operation_input.trim();

    let operation = match parse_operation(operation_input) {
        Some(op) => op,
        None => {
            println!("Invalid operation entered.");
            return;
        }
    };

    println!("Enter first number:");
    let mut first_number_input = String::new();
    io::stdin()
        .read_line(&mut first_number_input)
        .expect("Failed to read line");
    let x: f64 = match first_number_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered.");
            return;
        }
    };

    println!("Enter second number:");
    let mut second_number_input = String::new();
    io::stdin()
        .read_line(&mut second_number_input)
        .expect("Failed to read line");
    let y: f64 = match second_number_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered.");
            return;
        }
    };

    match calculate(operation, x, y) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
