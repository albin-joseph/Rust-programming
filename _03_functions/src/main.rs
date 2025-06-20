//In Rust hoisting feature available, in the file any where we can declare/impolemnt the variable/functions
//function declare with fn keyword, followed by function name in snake case
//variable or function name should be in snake case
//parameters we can pass to the functions by variable reference 

fn main() {
    print_hello_world();
    let int_value: i32 = 2;
    let result: i32 = get_square_of_int(int_value);
    println!("Square of 2 is: {}", result);
    human_id("Albin", 35, 165.0);

    let weight: f64 = 60.5;
    let height: f64 = 1.65;

    let bmi: f64 = calculate_bmi(weight, height);
    println!("Your BMI is {:.2}", bmi)

}

fn print_hello_world() {
    println!("Hello, world! from print_hello_world()");
}

fn get_square_of_int (value: i32) -> i32 {
    value * value
}

fn human_id(name: &str, age: i32, hieght: f64) {
    println!("My name is {}, I have {} years old and my height is {}", name, age, hieght);
}

//Expression & Statement
//Expression: anything that returns a value
//Statement: anything that does not returns a value
//Almost all statements in Rust end with ;

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg/(height_m*height_m)
}