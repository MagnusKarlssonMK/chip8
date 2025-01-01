use chip8::Config;
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Argument error: {err}");
        process::exit(1);
    });

    if let Err(e) = config.run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
