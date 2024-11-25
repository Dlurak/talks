// imports mit `use`
use std::fmt;

trait ColoredText {
    fn color(value: fmt::Display) -> &str
}

// tuple struct
// gibt auch normale
struct RGB(u8, u8, u8)

enum Color {
    Red,
    Green,
    Blue,
    Custom(RGB),
}

impl From<RGB> for Color {
    fn from(value: RGB) -> Self {
        Color::Custom(value)
    }
}

impl From<Color> for RGB {
    fn from(value: Color) -> RGB {
        match value {
            Red => RGB(255, 0, 0),
            Green => RGB(0, 255, 0),
            Blue => RGB(0, 0, 255),
            Custom(RGB) => RGB,
        }
    }
}
