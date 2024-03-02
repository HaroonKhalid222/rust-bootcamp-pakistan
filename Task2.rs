use std::io;

// Enum representing different arithmetic operations
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Function to calculate the result based on the provided Operation enum
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => {
            if y != 0.0 {
                x / y
            } else {
                println!("Error: Cannot divide by zero!");
                f64::NAN // Return NaN in case of division by zero
            }
        }
    }
}

fn main() {
    // Prompt the user for input
    println!("Enter the first number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let first_number: f64 = input.trim().parse().expect("Invalid number");

    println!("Enter the operation (+, -, *, /):");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation: char = input.trim().chars().next().expect("Invalid operation");

    println!("Enter the second number:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let second_number: f64 = input.trim().parse().expect("Invalid number");

    // Create an Operation enum instance based on user input
    let user_operation = match operation {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Subtract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => {
            println!("Error: Invalid operation");
            return;
        }
    };

    // Call the calculate function and print the result
    let result = calculate(user_operation);
    println!("Result: {}", result);
}
