use std::collections::HashMap;

struct Contents {
    content: String,
}

fn main() {
    println!("==========Hashmap===========");
    let mut lockers = HashMap::new();
    lockers.insert(1, Contents{content: "stuff".to_owned()});
    lockers.insert(2, Contents{content: "shirts".to_owned()});
    lockers.insert(3, Contents{content: "gym_shorts".to_owned()});

    for (locker_number, content) in lockers.iter(){
        println!("numder: {}, content: {:?}", locker_number, content.content);
    }
}
