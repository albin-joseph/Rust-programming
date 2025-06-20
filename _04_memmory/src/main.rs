
//Ownership, Borrowing and References
//What is Ownership
//Every value has a single owner [every variable has one value, and it is its owner]
//-----------------------
//Ownership Rules
//1.Each value in Rust has an owner
//2.There can be only one owner at a time
//3.When the owner goes out of scope, the value will be dropped.


fn main() {
    println!("Memmory management in RUST");
    //Rule1.
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of  '{}' is {}", s1, len);

    //In the above s1 is the owner of the string RUST and we are only passing the reference to the function
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

