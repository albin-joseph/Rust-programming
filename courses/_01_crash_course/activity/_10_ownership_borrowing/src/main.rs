
// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity
// * Create a function to display the id number

struct Grocery {
    id:i32,
    quantity:i32,
}

fn display_id(grocery: &Grocery) {
    println!("id: {:?}", grocery.id);
}

fn display_quantity(grocery: &Grocery) {
    println!("quantity: {:?}", grocery.quantity);
}

fn main() {
    println!("Ownership & Borrowing");

    let grocery = Grocery{
        id:1,
        quantity:75,
    };

    display_id(&grocery);
    display_quantity(&grocery);
}
