// Structs are used to name and package related values similar to tuple
fn main() {
    println!("Structs programs");

    //tuple
    let rect: (i32, i32) = (200, 500);

    // Struct
    struct Book {
        title: String,
        author: String,
        pages: u32,
        available: bool,
    };

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    };

    let mut user1: User = User {
        active: true,
        username: String::from("albinjoseph"),
        email: String::from("albin@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("autor@gmail.com");
    println!("User email is {}", user1.email);

    //Struct can return from a function
    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            email,
            username,
            sign_in_count: 1
        }
    }

    //Create instance from other instances
    let user2 = User {
        email: String::from("abcd@gmail.com"),
        ..user1
    };

    println!("name: {}", user2.username );

    //Tuple Structs
    struct Color(i32,i32,i32);
    struct Point(i32,i32,i32);

    let black: Color = Color(0,0,0);
    let white: Color = Color(255,255,255);

    // unit_like structs
    struct AlwaysEqual;
    let subject:AlwaysEqual = AlwaysEqual;

    //println!("Subject: {}", subject);

}

