fn main() {
    println!("String Program");

    let sentence = "Hello my name is Albin Joseph";

    let words = sentence.split_whitespace();

    for word in words {
        println!("{:?}",word );
    }
}
