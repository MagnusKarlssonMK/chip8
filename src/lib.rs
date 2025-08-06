use clap::Parser;
use std::{error::Error, fs, path::PathBuf};

use chip8options::Chip8options;
use display::Display;
use emulator::Emulator;
use keyboard::Keyboard;

mod chip8options;
mod display;
mod emulator;
mod keyboard;
mod opcode;

struct Peripherals {
    display: display::Display,
    keyboard: keyboard::Keyboard,
}

impl emulator::System for Peripherals {
    fn update_screen(&mut self, display_output: &[bool]) {
        self.display.draw_screen(display_output);
    }

    fn get_key_event(&mut self) -> Option<emulator::KeyEvent> {
        self.keyboard.get_chip8_key_events()
    }
}

/// CHIP-8 emulator program
#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct Args {
    /// ROM file name
    #[arg(name = "FILE")]
    rom_file: PathBuf,
}

pub struct Config {
    rom_file: PathBuf,
    chip8_options: Chip8options,
}

impl Config {
    /// Builds the CHIP-8 configuration based on the input arguments.
    pub fn build(args: Args) -> Result<Config, &'static str> {
        // Read option configurations
        let chip8_options = Chip8options::get_options();

        Ok(Config {
            rom_file: args.rom_file,
            chip8_options,
        })
    }

    /// Starts running the emulator.
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        // Read ROM file
        let rom = fs::read(&self.rom_file)?.to_vec();

        let mut emulator = Emulator::new(&rom, &self.chip8_options);
        let sdl_context = sdl2::init()?;
        let mut peripherals = Peripherals {
            display: Display::new(&sdl_context, &self.chip8_options.display)?,
            keyboard: Keyboard::new(&sdl_context)?,
        };
        emulator.run(&mut peripherals);
        Ok(())
    }
}
