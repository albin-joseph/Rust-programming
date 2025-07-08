// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//name to print

enum Color {
        Red,
        Blue,
        Green,
        Black,
        White
    }

fn main() {
    println!("Enum program");
    let color_name = Color::Red;
    print_color_name(color_name);
}

fn print_color_name(color_name: Color) {
    match color_name {
            Color::Red => println!("Red"),
            Color::Blue => println!("Blue"),
            Color::Green => println!("Green"),
            Color::Black => println!("Black"),
            _ => println!("Any other color")
        }
}
