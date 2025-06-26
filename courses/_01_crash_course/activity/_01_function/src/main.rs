//Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the printin macro to display messages to the terminal

fn main() {
    let name = get_full_name("Albin".to_string(), "Joseph".to_string());
    println!("Full Name: {}", name);
}

fn get_full_name(first_name: String, last_name: String) -> String {
    println!("First Name: {}", first_name);
    println!("Last Name: {}", last_name);

    let full_name = first_name + " " + &last_name;
    full_name
}
