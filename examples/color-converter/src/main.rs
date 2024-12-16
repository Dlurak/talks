mod color;

use clap::Parser;

/// Shows a color
#[derive(Parser, Debug)]
#[command(
    name = "colorsh",
    version = "1.0",
    about = "Show colors on the terminal",
    long_about = None
)]
struct Args {
    /// The color to process, in "R,G,B" format
    #[arg(value_parser = parse_color)]
    color: color::Color,
}

/// Custom value parser for `Color`
fn parse_color(s: &str) -> Result<color::Color, String> {
    color::Color::try_from(s)
}

fn main() {
    let args = Args::parse();
    let (r, g, b) = args.color.to_rgb();

    println!(
        "\x1b[48;2;{};{};{}m\x1b[38;5;{}m{}\x1b[0m",
        r,
        g,
        b,
        u8::from(args.color.foreground()),
        args.color
    );
}
