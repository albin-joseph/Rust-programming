enum Access{
    Admin,
    Manager,
    User,
    Guest
}
fn main() {
    println!("<--------- Expressions --------->");
    let access_level = Access::Guest;
    let can_access_file = match access_level {
        Access::Admin => true,
        Access::Manager => true,
       _=>false,
    };

    println!("Can acces files: {}", can_access_file);
}
