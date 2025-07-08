// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value
//a boolean variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display
fn main() {
    let bool_value = true;
    match bool_value {
        true=>println!("it's true"),
        false=>println!("It's false")
    }
}
