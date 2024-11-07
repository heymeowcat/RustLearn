#[derive(Debug)]

struct Book {
    title: String,
    author: String,
    is_checked_out: bool,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, title: String, author: String) {
        let book = Book {
            title,
            author,
            is_checked_out: false,
        };
        self.books.push(book);
    }

    fn check_out_book(&mut self, title: &str) -> Result<(), String> {
        if let Some(book) = self.books.iter_mut().find(|b| b.title == title) {
            if book.is_checked_out {
                Err(format!("book '{}' is already checked out.", title))
            } else {
                book.is_checked_out = true;
                Ok(())
            }
        } else {
            Err(format!("book '{}' does not exist in the library.", title))
        }
    }

    fn return_book(&mut self, title: &str) -> Result<(), String> {
        if let Some(book) = self.books.iter_mut().find(|b| b.title == title) {
            if book.is_checked_out {
                book.is_checked_out = false;
                Ok(())
            } else {
                Err(format!("book '{}' was not checked out.", title))
            }
        } else {
            Err(format!("book '{}' does not exist in the library.", title))
        }
    }

    fn list_books(&self) {
        for book in &self.books {
            let status = if book.is_checked_out {
                "Checkedout"
            } else {
                "Available"
            };
            println!(
                "Title: '{}', Author: '{}', Status: {}",
                book.title, book.author, status
            );
        }
    }
}

fn main() {
    let mut library = Library::new();
    library.add_book("The Book 1".to_string(), "Author 1".to_string());
    library.add_book("The Book 2".to_string(), "Author 2".to_string());

    library.list_books();
    library.check_out_book("The Book 1").unwrap();
    library.list_books();
    library.return_book("The Book 1").unwrap();
    library.list_books();
}
