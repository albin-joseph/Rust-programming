
// Topic: Result
// Requirements:
// * Create an structure named 'Adult' that represents a person aged 21 or older:
// * The structure must contain the person's name and age
// * Implement Debug print functionality using 'derive'
// * Implement a 'new' function for the 'Adult' structure that returns a Result:
//* The ok variant should contain the initialized structure, but only 
//  if the person is aged 21 or older

//* The Err variant should contain a String (or &str) that explains why the structure could not be created
// * Instantiate two 'Adult' structures:
//* One should be aged under 21
//* One should be 21 or over
// * Use "match' to print out a message for each 'Adult':
//* For the Ok variant, print any message you want
//* For the Err variant, print out the error message

struct Adult {
    name:String,
    age:i32,
}

impl Adult {
    fn new (name: &str, age: i32) -> Result<Self, &str> {
        if age >= 21 {
            return Ok(Self {
                name: name.to_string(),
                age,
            });
        } else {
            return Err("The person's age is below 21".to_owned());
        }
    } 
}

fn main() {
    println!("Result Program");

    let x = Adult::new("Albin", 30);

    match x {
        Ok(person) => println!("Person => name: {:?} & age: {:?}", person.name, person.age),
        Err(e) => println!("{}", e),
    }

     let y = Adult::new("Emmanuel", 4);

    match y {
        Ok(person) => println!("Person => name: {:?} & age: {:?}", person.name, person.age),
        Err(e) => println!("{}", e),
    }
}
