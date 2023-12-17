use std::io;

fn main() {
    let value1: String = read_valid_input("Select first number:", validate_number);
    let operator: String = read_valid_input("Select operator:", validate_operator);
    let value2: String = read_valid_input("Select second number:", validate_number);

    let num1: f64 = value1.parse::<f64>().unwrap();
    let num2: f64 = value2.parse::<f64>().unwrap();

    let result: f64 = match calculate_result(num1, num2, &operator) {
        Some(r) => r,
        None => {
            println!("Invalid operation.");
            return;
        }
    };

    println!("*******************************");
    println!("The result is: {}", result);
    println!("*******************************");
}

fn read_valid_input<F>(prompt: &str, validator: F) -> String
where
    F: Fn(&str) -> bool,
{
    loop {
        println!("{}", prompt);
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if validator(&input) {
            return input.trim().to_string();
        }
    }
}

fn validate_number(input: &str) -> bool {
    input.trim().parse::<f64>().is_ok()
}

fn validate_operator(input: &str) -> bool {
    matches!(input.trim(), "+" | "-" | "*" | "/")
}

fn calculate_result(val1: f64, val2: f64, op: &str) -> Option<f64> {
    match op.trim() {
        "+" => Some(val1 + val2),
        "-" => Some(val1 - val2),
        "*" => Some(val1 * val2),
        "/" => {
            if val2 == 0.0 {
                println!("Cannot divide by zero");
                None
            } else {
                Some(val1 / val2)
            }
        }
        _ => None,
    }
}
