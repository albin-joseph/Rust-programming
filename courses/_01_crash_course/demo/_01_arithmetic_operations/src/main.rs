fn main() {
    let sum = 2 + 2;
    let value = 10 - 5;
    let div_result = 10/2;
    let mult = 5 * 2;

    println!("Sum: {}, Substarction: {}, divison: {}, multiplication: {}", sum, value, div_result, mult);

    println!("Sum: {}, Substarction: {}, divison: {}, multiplication: {}", addition(1,2), substraction(4,3), division(3,2), multiplication(5,2));

    let rem = 5%3;

    println!("Remainder: {}", rem);
}



fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn substraction(a: i32, b: i32) -> i32 {
    a - b
}

fn multiplication(a: i32, b: i32) -> i32 {
    a * b
}

fn division(a: i32, b: i32) -> i32 {
    a / b
}




