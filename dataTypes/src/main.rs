fn main() {
    //PRIMITIVE DATA TYPES

    //Int
    let x: i32 = -42;
    let y: u32 = 10;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);

    //Float
    let pi: f64 = 3.14;
    println!("pi value: {}", pi);

    //Boolean
    let is_raining: bool = true;
    println!("is raining?: {}", is_raining);

    //char
    let letter: char = 'a';
    println!("first letter of the alphabets: {}", letter);

    //COMPUND DATA TYPES
    //arrays, tuples, slices and strings

    //Arrays
    //Array should contain homogenious items
    //Array type should contain 2 values type and number of items
    //[<type>, <no of items>]
    let  numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "banana", "orange"] ;
    println!("Fruits Array : {:?}", fruits);
    println!("Fruits Array at 0 : {}", fruits[0]);

    //Tuples
    //Tuple should contain hetrogeneous types
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let my_mix_tuple: (&'static str, i32, bool, [i32; 5]) = ("Albin", 23, true, [1,2,3,4,5]);
    println!("My mixed tuple: {:?}", my_mix_tuple);

    //Slices: Dynamically sized elements contigenous allocation of memmory
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["elephent", "horse", "cow", "dog", "cat"];
    println!("Animal Slices: {:?}", animal_slices);

     let books_slices: &[&String] = &[&"elephent".to_string(), &"horse".to_string(), &"cow".to_string()];
    println!("Books Slices: {:?}", books_slices);

    //String vs String Slices (&str)
    let mut stone_cold: String = String::from("Hello, ");
    println!("Stone Cold syas: {}", stone_cold);

    stone_cold.push_str("Yeah!");

     println!("Stone Cold syas: {}", stone_cold);


}
