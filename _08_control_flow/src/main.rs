//Control flow in RUust
//1-consditions [Id...Else]
//2-Repeating actions [Loops]
fn main() {
    println!("Control Flow");

    let age : u16 = 18;
    if age >= 18 {
        println!("You can drive a car");
    } else {
         println!("You can't drive a car");
    }

    //Use if in a  let statemaent
    let condition = true;
    let number = if condition {5} else {6};
    println!("Number is {number}");


    //Loops
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The value of result is {result} and counter value is {counter}");

    //While loop

    let mut number: i32 = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("Hey!");

    // for loop

    let number_array: [i32; 5] = [1, 2,3,4,5];

    for element in number_array {
        println!("Current number is {element}");
    }

}
