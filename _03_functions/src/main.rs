//In Rust hoisting feature available, in the file any where we can declare/impolemnt the variable/functions
//function declare with fn keyword, followed by function name in snake case
//variable or function name should be in snake case
//parameters we can pass to the functions by variable reference 

fn main() {
    print_hello_world();
    let int_value = 2;
    let result = get_square_of_int(int_value);
    println!("Square of 2 is: {}", result);
}

fn print_hello_world() {
    println!("Hello, world! from print_hello_world()");
}

fn get_square_of_int (value: i32) -> i32 {
    value * value
}