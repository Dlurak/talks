---
title: Rust Cheatsheet
author: Dimi
---

# Clap Cheatsheet

```rust
use clap::{Parser, Subcommand};

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
    let args = Cli::parse();
    println!("{:?}", args);
}
```
