---
author: Dimi
title: 38c3 Workshop - Übungen
---

# Übungen

Bevor es an die Projekte gehen kann, muss man erstmal paar Grundlagen können, hier sind paar simple Übungen hierfür.


## Enums
```rust

#[derive(Debug, Default)]
#[allow(dead_code)]
enum ChocolateFlavor {
    #[default]
    Milk,
    Dark,
    White,
    Hazelnut,
    Mint,
}

fn prize(choco: ChocolateFlavor) -> f32 {
    match choco {
        ChocolateFlavor::Milk | ChocolateFlavor::Dark | ChocolateFlavor::White => 2.49,
        ChocolateFlavor::Hazelnut => 2.99,
        ChocolateFlavor::Mint => 3.99,
    }
}

fn main() {
    dbg!(prize(ChocolateFlavor::default()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prize() {
        assert_eq!(prize(ChocolateFlavor::Milk), 2.49);
        assert_eq!(prize(ChocolateFlavor::Dark), 2.49);
        assert_eq!(prize(ChocolateFlavor::White), 2.49);
        assert_eq!(prize(ChocolateFlavor::Hazelnut), 2.99);
        assert_eq!(prize(ChocolateFlavor::Mint), 3.99);
    }
}
```

## Hello_world
```rust

fn main() {
    let name = "Dimi";
    let mut age = 17;
    println!("Ich bin {name}(age)");
    age += 1;
    println!("Am 39C3 aber schon {age}");
}
```

## If
```rust

fn temperature() -> f32 {
    let env_var = std::env::var("temperature").ok();
    let provided_f32 = env_var.and_then(|v| v.parse().ok());
    provided_f32.unwrap_or(12.0)
}

fn main() {
    let temp = temperature();
    let msg = if temp < 0.0 {
        "Kalt"
    } else if temp < 50.0 {
        "Warm"
    } else {
        "Heiß"
    };

    println!("{msg}");
}
```

## Structs
```rust

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

    fn check_out(&mut self) -> Result<(), ()> {
        if self.is_checked_out {
            Err(())
        } else {
            self.is_checked_out = true;
            Ok(())
        }
    }

    fn return_book(&mut self) -> Result<(), ()> {
        if !self.is_checked_out {
            Err(())
        } else {
            self.is_checked_out = false;
            Ok(())
        }
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for Book {
    fn summarize(&self) -> String {
        let checked_out = if self.is_checked_out {
            "Checked out"
        } else {
            "Available"
        };

        format!(
            "{} by {} - {}",
            self.title,
            self.author,
            checked_out
        )
    }
}

fn main() {
    println!(Book::new("Rust", "Steve").summarize());
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_book() {
        let mut book = Book::new("Rust", "Steve");
        assert_eq!(Book {
            title: String::from("Rust"),
            author: String::from("Steve"),
            is_checked_out: false,
        }, book);
        assert_eq!(
            book.summarize(),
            "Rust by Steve - Available"
        );
        assert_eq!(
            book.check_out(),
            Ok(())
        );
        assert_eq!(
            book.check_out(),
            Err(())
        );
        assert_eq!(
            book.summarize(),
            "Rust by Steve - Checked out"
        );
        assert_eq!(
            book.return_book(),
            Ok(())
        );
        assert_eq!(
            book.return_book(),
            Err(())
        );
    }
    
}
```
