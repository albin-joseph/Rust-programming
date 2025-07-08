// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color {
    Red,
    Green,
    Blue,
}

struct Box {
    //dimesnion (L,W,H)
    dimensions: (f64,f64,f64),
    color: Color,
    weight: f64,
}

impl Box {
    fn create_new_box()->Self {
        Self{
            dimensions:(10.0, 15.0, 12.5),
            color: Color::Blue,
            weight: 7.5,
        }
    }
    fn print_box_characteristics(&self) {
        println!("Box dimesion: (L:{:?}, W:{:?}, H:{:?})", self.dimensions.0, self.dimensions.1, self.dimensions.2);
        println!("Box weight: {:?}", self.weight);
        match self.color {
            Color::Red => println!("Box color: Red"),
            Color::Green => println!("Box color: Green"),
            Color::Blue => println!("Box color: Blue"),
        }
    }
}

fn main() {
    println!("imp program");

    let _box = Box {
        dimensions:(15.0, 7.5, 10.0),
        color: Color::Red,
        weight: 3.5,
    };

    _box.print_box_characteristics();

    let blue_box = Box::create_new_box();
    blue_box.print_box_characteristics();
}
