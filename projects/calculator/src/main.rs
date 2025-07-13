use std::io;

fn main() {
    println!("🧮 Rust CLI Calculator");

    // Read first number
    let num1 = read_number("Enter the first number: ");

    // Read second number
    let num2 = read_number("Enter the second number: ");

    // Read operator
    println!("Choose an operator (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("Failed to read operator");
    let operator = operator.trim();

    // Perform calculation
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("❌ Error: Division by zero");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("❌ Invalid operator");
            return;
        }
    };

    println!("✅ Result: {} {} {} = {}", num1, operator, num2, result);
}

fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(n) => return n,
            Err(_) => println!("❌ Please enter a valid number"),
        }
    }
}
