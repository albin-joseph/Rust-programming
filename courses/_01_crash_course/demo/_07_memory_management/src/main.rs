struct Book {
     page_number:i32,
     rating:i32
}

fn print_page_number(book:&Book){
    println!("No of pages:{:?}", book.page_number);
}

fn print_rating(book:&Book) {
 println!("rating:{:?}", book.rating);
}

fn main() {
    println!("Memory Management Program - Ownership & Borrow");
    let book = Book {
        page_number: 250,
        rating: 4,
    };

    print_page_number(&book);
    print_rating(&book);
}
