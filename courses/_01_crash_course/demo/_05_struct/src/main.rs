struct GroceryItems {
   stock: i32,
   price: f64, 
}

fn main() {
    println!("struct programs");

    let cereal = GroceryItems {
        stock: 10,
        price: 2.99,
    };

    println!("stock: {:?}", cereal.stock);
    println!("price: {:?}", cereal.price);
}
