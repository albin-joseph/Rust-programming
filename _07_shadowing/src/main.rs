//Shadowing
//Shadowing is creating a new varibale with same name
//The old variable will be shadowing the new
fn main() {
    println!("Shadowing");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x in main function is {x}");
}
