fn main() {
    println!("repetition using while");
    let mut i = 1;
    while i<=3 {
        println!("{:?}", i);
        i = i + 1;
    }
    println!("*** Program to print from 5 to 1 ***");
    let mut i = 5;
    while i>0 {
        println!("{:?}", i);
        i = i - 1;
    }
    println!("done!")
}
