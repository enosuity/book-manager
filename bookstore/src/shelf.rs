use crate::book::Book;



pub struct Shelf {
  pub books: Vec<Book>
}

impl Shelf {
  pub fn new() -> Self {
    Self {
      books: vec![]
    }
  }

  pub fn add_book(&mut self, book: Book) {
    self.books.push(book);
  }

  pub fn listing(&self) {
    for book in &self.books {
      println!("Book: {0} is written by {1}", book.title, book.author);
    }
  }
    
}