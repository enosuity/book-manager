use bookstore::book;
use bookstore::shelf;
use author::author::Author;

fn main() {
  let book1 = book::Book::new("The Rust Lang", "Anuj");
  let book2 = book::Book::new("The Rust by Example", "Eno");

  let mut admira = shelf::Shelf::new();
  admira.add_book(book1);
  admira.add_book(book2);

  admira.listing();  

  let author = Author::new("Anuj Dhiman", "India");
  
  author.author_info()

}

