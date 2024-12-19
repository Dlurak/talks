#[derive(Debug, PartialEq, Eq)]
enum BookError {
    AlreadyCheckedOut,
    NotCheckedOut,
}

#[derive(Debug, PartialEq, Eq)]
struct Book {
    title: String,
    author: String,
    is_checked_out: bool,
}

#[allow(dead_code)]
impl Book {
    fn new<T: Into<String>, A: Into<String>>(title: T, author: A) -> Self {
        Self {
            title: title.into(),
            author: author.into(),
            is_checked_out: false,
        }
    }

    fn check_out(&mut self) -> Result<(), BookError> {
    }

    fn return_book(&mut self) -> Result<(), BookError> {
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

// Summary für Book implementieren
// tipp: `format!`

fn main() {
    println!("{}", Book::new("Just for fun", "Linus Torvalds").summarize());
}

// Der Test überprüft Deinen Code, nciht ändern
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_book() {
        let mut book = Book::new("Rust", "Steve");
        assert_eq!(
            Book {
                title: String::from("Rust"),
                author: String::from("Steve"),
                is_checked_out: false,
            },
            book
        );
        assert_eq!(book.summarize(), "Rust by Steve - Available");
        assert_eq!(book.check_out(), Ok(()));
        assert_eq!(book.check_out(), Err(BookError::AlreadyCheckedOut));
        assert_eq!(book.summarize(), "Rust by Steve - Checked out");
        assert_eq!(book.return_book(), Ok(()));
        assert_eq!(book.return_book(), Err(BookError::NotCheckedOut));
    }
}
