use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
    #[arg(short, long)]
    debug: String
}

#[derive(Subcommand, Debug)]
enum Commands {
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
    let args = Cli::parse();
    println!("{:?}", args);
}
