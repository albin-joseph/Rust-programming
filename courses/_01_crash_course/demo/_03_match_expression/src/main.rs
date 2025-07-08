fn main() {
    println!("Match Expressions");

    let some_bbol = true;
    match some_bbol {
        true => println!("It's true"),
        false => println!("it's false"),
    }

    let some_int = 4;

    match some_int {
        1 => println!("Its 1"),
        2 => println!("Its 2"),
        3 => println!("Its 3"),
        _ => println!("Its something else")
    }

    let my_name = "albin";
    match my_name {
        "albin" => println!("That is my name"),
        "anu" => println!("Hi anu"),
        "emmanuel" => println!("Hello emmanuel"),
        "rebecca" => println!("Hello rebecca"),
        _=>println!("nice to meet you")
    }
}
