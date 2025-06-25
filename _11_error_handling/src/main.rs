fn main() {
    println!("Error handling");

    //1
    enum Option<T> { //Define the genric Option type
        Some(T), //Represent a value
        None, // Respresent a no value
    }
    let result = divide(10.0,0.0);
    match(result) {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by 0"),
    }
    //2
    enum Result<T, E> { //Define genric Result type
        Ok(T), //Represents a value
        Err(E), //Represents an error
    }

    match divide_result(100.0,0.0){
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }


}

fn divide(numerator: f64, denominator: f64) ->Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}



fn divide_result(numerator: f64, denominator: f64) ->Result<f64, String> {
    if denominator == 0.0 {
        Err(String::from("cannot divide by 0"))
    } else {
        Ok(numerator / denominator)
    }
}