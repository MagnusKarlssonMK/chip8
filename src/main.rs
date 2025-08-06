use chip8::{Args, Config};
use clap::Parser;
use std::process;

fn main() {
    let config = Config::build(Args::parse()).unwrap_or_else(|err| {
        eprintln!("Argument error: {err}");
        process::exit(1);
    });

    if let Err(e) = config.run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
