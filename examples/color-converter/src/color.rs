use regex::Regex;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    RGB(u8, u8, u8),
    HSL(u16, u8, u8),
}

#[derive(Default, Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum AnsiColor {
    #[default]
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

macro_rules! impl_from_ansi {
    ($type:ty) => {
        impl From<AnsiColor> for $type {
            fn from(color: AnsiColor) -> Self {
                match color {
                    AnsiColor::Black => 0 as $type,
                    AnsiColor::Red => 1 as $type,
                    AnsiColor::Green => 2 as $type,
                    AnsiColor::Yellow => 3 as $type,
                    AnsiColor::Blue => 4 as $type,
                    AnsiColor::Magenta => 5 as $type,
                    AnsiColor::Cyan => 6 as $type,
                    AnsiColor::White => 7 as $type,
                }
            }
        }
    };
}

impl_from_ansi!(u8);
impl_from_ansi!(u16);
impl_from_ansi!(u32);
impl_from_ansi!(u64);
impl_from_ansi!(u128);
impl_from_ansi!(i8);
impl_from_ansi!(i16);
impl_from_ansi!(i32);
impl_from_ansi!(i64);
impl_from_ansi!(i128);
impl_from_ansi!(usize);
impl_from_ansi!(isize);
impl_from_ansi!(f32);
impl_from_ansi!(f64);

impl Color {
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::RGB(r, g, b) => (*r, *g, *b),
            Color::HSL(h, s, l) => {
                let h = *h as f32;
                let s = (*s as f32) / 100.0;
                let l = (*l as f32) / 100.0;

                let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
                let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
                let m = l - c / 2.0;

                let (r, g, b) = if h >= 0.0 && h < 60.0 {
                    (c, x, 0.0)
                } else if h >= 60.0 && h < 120.0 {
                    (x, c, 0.0)
                } else if h >= 120.0 && h < 180.0 {
                    (0.0, c, x)
                } else if h >= 180.0 && h < 240.0 {
                    (0.0, x, c)
                } else if h >= 240.0 && h < 300.0 {
                    (x, 0.0, c)
                } else {
                    (c, 0.0, x)
                };

                (
                    ((r + m) * 255.0).round() as u8,
                    ((g + m) * 255.0).round() as u8,
                    ((b + m) * 255.0).round() as u8,
                )
            }
        }
    }

    pub fn foreground(&self) -> AnsiColor {
        let (r, g, b) = self.to_rgb();
        let brightness = r as u32 * 299 + g as u32 * 587 + b as u32 * 114;
        if brightness > 125_000 {
            AnsiColor::Black
        } else {
            AnsiColor::White
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = String;

    fn try_from(str: &str) -> Result<Self, Self::Error> {
        parse_color_variant(
            str,
            Regex::new(
                r"(?i)^(?:rgb\s*\(?)(\d{1,3})(?:,|,?\s+)(\d{1,3})(?:,|,?\s+)(\d{1,3}),?\)?$",
            )
            .unwrap(),
            |n, _| {
                if (0..=255).contains(&n) {
                    Ok(n)
                } else {
                    Err(format!("number {n} too large, maximum 255"))
                }
            },
            |(r, g, b)| Color::RGB(r as u8, g as u8, b as u8),
        )
        .or(parse_color_variant(
            str,
            Regex::new(
                r"(?i)^(?:hsl\s*\(?)(\d{1,3})(?:,|,?\s+)(\d{1,3})(?:,|,?\s+)(\d{1,3}),?\)?$",
            )
            .unwrap(),
            |n, i| {
                let max = if i == 0 { 360 } else { 100 };
                if (0..=max).contains(&n) {
                    Ok(n)
                } else {
                    Err(format!("number {n} too large, maximum {max}"))
                }
            },
            |(h, s, l)| Color::HSL(h, s as u8, l as u8),
        ))
        .unwrap_or(Err(String::from("Unrecorgnized color format")))
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::RGB(r, g, b) => write!(f, "RGB({r},{g},{b})"),
            Color::HSL(h, s, l) => write!(f, "HSL({h},{s},{l})"),
        }
    }
}

fn parse_color_variant<V, C, R>(
    str: &str,
    pattern: Regex,
    validator: V,
    constructor: C,
) -> Option<Result<R, String>>
where
    V: Fn(u16, usize) -> Result<u16, String>,
    C: Fn((u16, u16, u16)) -> R,
{
    let matches: Vec<_> = pattern
        .captures_iter(str)
        .map(|c| c.extract::<3>().1)
        .flatten()
        .collect();

    if matches.is_empty() {
        return None;
    }

    let numbers = matches
        .iter()
        .enumerate()
        .map(|(i, &m)| -> Result<u16, String> {
            let num = m.parse::<u16>().map_err(|e| format!("{}: {}", m, e))?;
            validator(num, i)
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|nums| (nums[0], nums[1], nums[2]));

    Some(numbers.map(constructor))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tryfrom() {
        assert_eq!(Ok(Color::RGB(0, 0, 0)), Color::try_from("rgb 0 0  0"));
        assert_eq!(
            Ok(Color::RGB(255, 255, 255)),
            Color::try_from("rgb 255 255, 255")
        );
        assert_eq!(Ok(Color::RGB(20, 42, 0)), Color::try_from("RGB(20, 42, 0)"));

        assert_eq!(Ok(Color::HSL(20, 42, 0)), Color::try_from("HSL(20, 42, 0)"));
        assert_eq!(
            Ok(Color::HSL(120, 90, 0)),
            Color::try_from("hsl(120, 90, 0)")
        );

        assert_eq!(
            Err(String::from("Unrecorgnized color format")),
            Color::try_from("rgba(120, 90, 0)")
        );
        assert_eq!(
            Err(String::from("Unrecorgnized color format")),
            Color::try_from("rgba(120, 90, 0)")
        );
    }
}
