// Topic: Data management using tuples
//
// Requirements:
// Print whether the y-value of a cartesian coordinate is
//greater than 5, less than 5, or equal to 5
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn get_coordinates() -> (i16,i16) {
    let coordinates = (5, 8);
    coordinates
}
fn main() {
   let my_coordinate_value = get_coordinates();
   if my_coordinate_value.1 > 5 {
        println!("Y > 5");
    } else if my_coordinate_value.1 < 5 {
        println!("Y < 5");
    } else {
        println!("Y = 5");
    }
}
