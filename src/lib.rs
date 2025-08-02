use std::{env, error::Error, fs};

use chip8options::Chip8options;
use display::Display;
use emulator::Emulator;
use keyboard::Keyboard;

mod chip8options;
mod display;
pub mod emulator;
mod keyboard;
pub mod opcode;

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

pub struct Config {
    rom: Vec<u8>,
    chip8_options: Chip8options,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let rom_file = match args.next() {
            Some(arg) => arg,
            None => {
                return Err(
                    "Use file name of ROM file to run (no file extension) as first argument.",
                );
            }
        };

        // Read input file
        let cwd = env::current_dir().unwrap();
        let filename = cwd.join("rom_files").join(format!("{}.ch8", &rom_file));

        let rom = match fs::read(filename) {
            Ok(f) => f.to_vec(),
            Err(_) => {
                return Err("File not found");
            }
        };

        // Read option configurations
        let chip8_options = Chip8options::get_options();

        Ok(Config { rom, chip8_options })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut emulator = Emulator::new(&self.rom, &self.chip8_options);
        let sdl_context = sdl2::init()?;
        let mut peripherals = Peripherals {
            display: Display::new(&sdl_context, &self.chip8_options.display)?,
            keyboard: Keyboard::new(&sdl_context)?,
        };
        emulator.run(&mut peripherals);
        Ok(())
    }
}
