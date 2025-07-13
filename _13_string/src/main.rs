fn main() {
    println!("String Program");

    let sentence = "Hello my name is Albin Joseph";

    let words = sentence.split_whitespace();

    for word in words {
        println!("{:?}",word );
    }

    reverse_string("Albin");
    reverse_string("Anu Jose");
    reverse_string("Emmanuel Joseph Albin");
    reverse_string_optimised("Emmanuel Joseph Albin");
}

fn reverse_string(text: &str)  {
    let mut str_array: Vec<char> = Vec::new();
    for letter in text.chars() {
        str_array.push(letter);
    }

   let mut reverse_str = String::new();
   while str_array.len() > 0 {
    match str_array.pop() {
        Some(value) => {reverse_str.push(value)},
        None => println!("Something went wrong")
    }
   }

   println!("{:?}", reverse_str);
}

//Complexity is O(n)
fn reverse_string_optimised(text: &str) {
    let mut reverse_str = String::with_capacity(text.len());
    for ch in text.chars().rev() {
        reverse_str.push(ch);
    }
    println!("{:?}", reverse_str);
}
