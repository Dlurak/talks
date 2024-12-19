---
title: Rust Cheatsheet
author: Dimi
---

# Rust Cheatsheet

## Basics

| Keyword   | Beschreibung                                                               |
| --------- | -------------------------------------------------------------------------- |
| `fn`      | Funktion                                                                   |
| `let`     | Unveränderbare Variable, der Type kann meistens automatisch erkannt werden |
| `let mut` | Veränderbare Variable                                                      |

- **`main` Funktion**: Einstieg ins Programm

## Variablen

Normalerweise **unveränderbar**:

```rust
let x = 42;
x = 12;
// Fehler: x ist unveränderbar
```

`mut` Keyword für **veränderbar**:

```rust
let mut x = 42;
x = 12;
```

Expliziter Type:

```rust
let x: u8 = 42;
```

## Enums

```rust
#[derive(Debug)]
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
    Custom(RGB)
}
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

## `if let` und `match`

- Fachwort: **Patternmatching**
- Daten aus einer `enum`-Variante auslesen
- `match` ist vergleichbar mit `switch case` Statements aus JS oder C
- `match` muss alle Möglichkeiten abdecken

```rust
if let Color::Custom(color_name) = my_color {
    println!("{:?}", color_name);
}

let color_name = match my_color {
    Color::Red => "rot",
    Color::Green => "grün",
    Color::Blue => "blau",
    Color::Custom(name) => name,
};
let color_name = match my_color {
    Color::Red => "rot",
    _ => "Nicht rot"
};
```

## Traits und `impl`

```rust
impl Pair {
    fn new(first: i32, second: i32) -> Pair {
        Pair(first, second)
    }
    fn swap(&mut self) {
        let Pair(a, b) = *self;
        *self = Pair(b, a);
    }
}

impl Into<Point> for Pair {
    fn into(self) -> Point {
        Point { x: self.0, y: self.1 }
    }
}

// funktioniert statisch auf dem type
let mut pair = Pair::new(42, 12);
// funktioniert nur auf initialisierten Instanzen vom struct
pair.swap();
let point: Point = pair.into();
```

## Fehlerhandling

Rust hat **keine** klassischen Fehler!
Es gibt das `enum Result`:

```rust
let result: Result<i32, String> = Ok(42);
match result {
    Ok(value) => println!("Erfolg: {}", value),
    Err(error) => println!("Fehler: {}", error),
}

fn success() -> Result<Vec<i32>, String> {
    vec![1, 2, 3, 4]
}

fn error() -> Result<i32, String> {
    Err(String::from("Computer sagt nein"))
}
```

## `Option` (null-safety)

Rust hat **kein** klassisches `null` wie JavaScript oder C
Es gibt das `enum Option`:

```rust
let option: Option<i32> = Some(42);
match option {
    Some(value) => println!("Daten sind da: {}", value),
    None => println!("Keine Daten"),
}
```
