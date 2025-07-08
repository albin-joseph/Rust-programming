enum Direction {
    Left,
    Right,
    Up,
}
fn main() {
    println!("Enumeration program");
    let go = Direction::Right;
    match go {
        Direction::Left => println!("go left"),
        Direction::Right => println!("go right"),
        _=>println!("go up or down")
    }
}
