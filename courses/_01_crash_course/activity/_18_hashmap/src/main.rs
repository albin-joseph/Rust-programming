use std::collections::HashMap;
// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
// * 5 Chairs
// * 3 Beds
// * 2 Tables
// * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
fn main() {
    println!("=============HashMap===========");

    let mut stocks = HashMap::new();
    stocks.insert("Chairs", 5);
    stocks.insert("Beds", 3);
    stocks.insert("Tables", 2);
    stocks.insert("Couches", 0);

    let mut total: i32 = 0;

    for (item, count) in &stocks {
        if *count > 0 {
            println!("{} : {}", item, count);
            total += count;
        } else {
            println!("{} is out of stock", item);
        }
    }

    println!("Total items in stock: {}", total);
}