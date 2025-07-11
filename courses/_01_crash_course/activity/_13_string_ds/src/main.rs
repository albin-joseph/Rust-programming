//Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for.. in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    favourite_color: String,
}

fn print_name(name: &str) {
    println!("name: {:?}", name);
}

fn print_color(color: &str) {
    println!("color: {:?}", color);
}

fn main() {
    println!("String DS Activity");

    let name = "Anu Jose".to_owned();

    let store = vec![
        Person {
            name: String::from("Albin Joseph"),
            age: 35,
            favourite_color: "Red".to_owned(),
        },
        Person {
            name: String::from("Anu Jose"),
            age: 31,
            favourite_color: "Blue".to_owned(),
        },
        Person {
            name: String::from("Emmanuel Joseph Albin"),
            age: 5,
            favourite_color: "Yellow".to_owned(),
        },
    ];

    for item in store {
        if item.name == name {
            print_name(&item.name);
            println!("age:{:?}", item.age);
            print_color(&item.favourite_color);
        }
    }
}
