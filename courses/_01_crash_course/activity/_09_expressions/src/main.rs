//Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is › 100
// * Print "its small" if a variable is ‹= 100
//
// Notes:
// * Use a boolean variable set to an expression 
// that determines whether the value is >100 or <=100
// * Use a function to print the messages
//'* Use a match expression to determine which message
// to print

fn main() {
    println!("<---------- Expressions --------->");
    let value:i64 = 90;
    let is_gt_100 = value > 100;
    print_message(is_gt_100);
}

fn print_message(gt_100: bool) {
    match gt_100 {
        true => println!("its big"),
        false => println!("its small")
    }
}

