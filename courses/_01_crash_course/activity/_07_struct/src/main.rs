
//Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// Use a match expression to print the drink flavour
enum Flavour {
    Lemon,
    Orange,
    Apple,
    Pista,
}

struct Drink {
    flavor:Flavour,
    ounces:f64,
}

fn print_drink(drink: Drink) {
    match drink.flavor {
        Flavour::Lemon => println!("Lemon"),
        Flavour::Orange => println!("Orange"),
        Flavour::Apple => println!("Apple"),
        Flavour::Pista => println!("Pista"),
    }

    println!("{}", drink.ounces);
}

fn main() {
    println!("struct program");

    let my_drink: Drink = Drink{
        flavor: Flavour::Pista,
        ounces: 0.750,
    };

    print_drink(my_drink);
}
