const helloWorld = `
fn main() {
	let name = "David";
	println!("Hallo {name}");
}
`

const helloWorldMut = `
fn main() {
	let mut name = "David";
	name = "Olivia";
	println!("Hallo {name}");
}
`

const init = `
$ cargo init
$ cargo run
`

const func = `
fn add(a: i32, b: i32) -> (i32, i32) {
	(a + b, b)
}

let (sum, b) = add(2, 5);
//   7    5
`;

const ifs = `
if age >= 18 {
	println!("Du bist volljährig")
} else {
	println!("Du bist nicht volljährig")
}
`

const ifAssignments = `
let msg = if age >= 18 {
	"Du bist volljährig"
} else {
	"Du bist nicht volljährig"
};
`

const enums = `
enum Color {
	Red,
	Blue,
	Green
}
`

const enumsData = `
enum Color {
	Red,
	Blue,
	Green,
	Custom { r: u8, g: u8, b: u8 }
}
`

const optionResult = `
fn http_status() -> Result<u16, MyError> {}
fn shortend(str: &str) -> Option<char> {}
`

const optionResultDef = `
Ok(0)
Err(ColorErr::UnrecognizedFmt)

Some(12)
None
`

const match = `
match color {
	Color::Red => println!("RGB(255, 0, 0)"),
	Color::Blue => println!("RGB(0, 0, 255)"),
	Color::Green => println!("RGB(0, 255, 0)"),
	Color::Custom { r, g, b } => println!("RGB({r}, {g}, {b})"),
}
`

const matchRes = `
use std::env;

match env::var("EDITOR") {
	Ok("nvim") => println!("Du nutzt Neovim!!!"),
	Ok(name) => println!("{}", name),
	Err(env::VarError::NotPresent) => println!("idk was du nutzt"),
	Err(env::VarError::NotUnicode(_)) => println!("Kein unicode"),
}
`;

const struct = `
struct Person {
	name: String,
	age: u8,
	favourite_color: Color,
}
`

const structDeriveImpl = `
#[derive(Default, PartialEq, Eq)]
struct Person {
	name: String,
	age: u8,
	favourite_color: Color,
}

impl Person {
	fn new(name: String, age: u8, color: Color) -> Self {
		Person {
			name,
			age,
			favourite_color: color
		}
	}
}
`

const basic = `
use clap::Parser;

#[derive(Parser)]
struct Cli {
	#[arg(short, long, default_value_t = true)]
	verify: bool,

	#[arg(short, long, default_value_t = false)]
	verbose: bool,
}

fn main() {
	let args = Cli::parse();
}
`

const subcommands = `
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Commands {
	Init,
	#[command(alias = "information")]
	Info { complete: Option<bool> }
}

#[derive(Parser)]
struct Cli {
	#[command(subcommands)]
	cmd: Commands,
	#[arg(short, long, default_value_t = false)]
	verbose: bool,
}

fn main() {
	let args = Cli::parse();
}
`

const clap = { basic, subcommands }

export const samples = {
	helloWorld,
	helloWorldMut,
	init,
	func,
	ifs,
	ifAssignments,
	enums,
	enumsData,
	optionResult,
	optionResultDef,
	match,
	matchRes,
	struct,
	structDeriveImpl,
	clap,
};
