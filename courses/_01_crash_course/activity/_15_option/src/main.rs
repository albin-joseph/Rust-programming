// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option‹ i32>

struct Student {
    name: String,
    locker: Option<i32>
}

fn main() {
    println!("<=============== Option ===============>");

    let s1_locker = Student {
        name: "s1".to_owned(),
        locker: Some(1234),
    };

    let s2_locker = Student {
        name: "s2".to_owned(),
        locker: None
    };

    println!("Student name: {:?}", s1_locker.name);
    match s1_locker.locker {
        Some(locker) => println!("Locker key: {:?}", locker),
        None => println!("No locker assigned"),
    }

    println!("Student name: {:?}", s2_locker.name);
    match s2_locker.locker {
        Some(locker) => println!("Locker key: {:?}", locker),
        None => println!("No locker assigned"),
    }

}
