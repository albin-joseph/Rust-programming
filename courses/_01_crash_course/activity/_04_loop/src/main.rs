fn main() {
    println!("Repetive loop statement");
    let mut i = 3;
    loop {
        println!("{:?}", i);
        i = i -1;
        if i==0 {
            break;
        }
    }
    println!("done!")
}
