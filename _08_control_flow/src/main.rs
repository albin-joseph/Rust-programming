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
}
