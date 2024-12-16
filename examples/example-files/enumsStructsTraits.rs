// imports mit `use`
use std::fmt;

#[allow(dead_code)]
trait ColoredText {
    fn color(value: dyn fmt::Display) -> &'static str;
}

// tuple struct
// gibt auch normale
#[derive(Debug)]
struct RGB(u8, u8, u8);

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(RGB),
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RGB({}, {}, {})", self.0, self.1, self.2)
    }
}

impl From<Color> for RGB {
    fn from(value: Color) -> RGB {
        match value {
            Color::Red => RGB(255, 0, 0),
            Color::Green => RGB(0, 255, 0),
            Color::Blue => RGB(0, 0, 255),
            Color::Custom(rgb) => rgb,
        }
    }
}

fn main() {
    println!("Color::Red {}", RGB::from(Color::Red));
    println!(
        "Color::Custom {}",
        RGB::from(Color::Custom(RGB(255, 80, 83)))
    );
}
