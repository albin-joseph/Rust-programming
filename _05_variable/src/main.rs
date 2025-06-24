//Variable and mutability
//Variable is immutable by default; if a value assigned to it we cannot chnage it

fn main() {
    println!("Variable & mutability");
    let mut a: u16 = 5;
    println!("The value of  is {}", a);
    a = 10;
    println!("The value of  is {}", a);
    
}
