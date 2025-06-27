// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?)" token in the printin macro to display the result
fn main() {
    println!("<---------Basic Math Program--------->");
    let result = sum(3, 5);
    display_result(result);

}

fn sum(a:i32, b:i32) -> i32 {
    a + b
}

fn display_result(result: i32) {
    println!("Result: {:?}", result);
}