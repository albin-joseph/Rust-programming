//Ownership, Borrowing and References
//What is Ownership
//Every value has a single owner [every variable has one value, and it is its owner]
//-----------------------
//Ownership Rules
//1.Each value in Rust has an owner
//2.There can be only one owner at a time
//3.When the owner goes out of scope, the value will be dropped.

fn main() {
    println!("Memory management in RUST");
    
    //Rule1.
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of  '{}' is {}", s1, len);
    //In the above s1 is the owner of the string RUST and we are only passing the reference to the function

    //Rule 2
    let s2 = s1;
    println!("Value of s2: {}", s2);
    // println!("values s1: {} and s2: {}", s1, s2);
    //In the above code s1 value moved to s2 after that s1 dropped from the memory.
    //So if we try to access s1 will throw error

    //Rule3
    //If we try to do anything outside the scope will throw error

    safety_fn();

    let mut account: BankAccount = BankAccount{
        owner: "Albin".to_string(),
        balance: 505.50,
    };

    //Immutable borrow to check balance
    account.check_balance();

    //Mutable borrow to withdraw money
    account.withdraw(45.50);

     //Immutable borrow to check balance
    account.check_balance();

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

//Safety and performance
//References: Enable you to borrow values without taking ownership
//Immutable Reference: It doesn't allow you to modify the data
//Mutable Reference: It allows you to modify the data
fn safety_fn() {
    let mut _x: i32 = 5;
    let _r: &mut i32 = &mut _x;
    *_r += 1;
    *_r -= 3;

    // Print the mutable reference first, then drop it
    println!("Value of _r: {}", _r);
    
    // Now we can access _x again since _r is no longer used
    println!("Value of _x: {}", _x);
}

struct BankAccount {
    owner: String,
    balance: f64
}

impl BankAccount {
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount
    }

    fn check_balance(&self) {
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}