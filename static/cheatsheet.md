---
title: Rust Cheatsheet
author: Dimi
---

# Rust Cheatsheet

## Basics

| Keyword   | Beschreibung                                                                |
| --------- | --------------------------------------------------------------------------- |
| `fn`      | Funktion                                                                    |
| `let`     | Unveränderbare variabele, der Type kann meistens automatisch erkannt werden |
| `let mut` | Veränderbare variabele                                                      |

- **`main` Funktion**: Einstieg ins Programm

## Enums

```rust
enum Color {
    Red,
    Green,
    Blue
}
```

Daten in einem `enum`-Wert

```rust
enum Color {
    Red,
    Green,
    Blue,
    Custom(&str)
}
```

`if let` und `match`

```rust
if let Color::Custom(color_name) = my_color {
    println!("{}", color_name);
}

let color_name = match my_color {
    Color::Red => "red",
    Color::Green => "green",
    Color::Blue => "blue",
    Color::Custom(name) => name,
};
```

## Structs

```rust
struct Unit;

#[derive(Debug)]
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
```

## Traits und `impl`

```rust
impl Pair {
    fn new(first: i32, second: i32) -> Pair {
        Pair(first, second)
    }
}

impl Into<Point> for Pair {
    fn into(self) -> Point {
        Point { x: self.0, y: self.1 }
    }
}
```

# Clap Cheatsheet

```rust
use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Commands,
    #[arg(short, long, default_value_t = false)]
    debug: bool
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Init,
    #[command(alias = "information")]
    Info(InfoArgs),
}

#[derive(Parser, Debug)]
struct InfoArgs {
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    Cli::parse();
}
```

# Quellen

- [Structs (Rust by example)](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
