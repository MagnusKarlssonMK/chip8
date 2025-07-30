use std::{env, error::Error, fs};

use display::Display;
use emulator::Emulator;
use keyboard::Keyboard;

mod display;
pub mod emulator;
mod keyboard;
pub mod opcode;

struct Peripherals {
    display: display::Display,
    keyboard: keyboard::Keyboard,
}

const DISPLAY_WIDTH: u8 = 64;
const DISPLAY_HEIGHT: u8 = 32;

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

        Ok(Config { rom })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        let mut emulator = Emulator::new(&self.rom, DISPLAY_WIDTH, DISPLAY_HEIGHT);
        let sdl_context = sdl2::init()?;
        let mut peripherals = Peripherals {
            display: Display::new(&sdl_context, DISPLAY_WIDTH, DISPLAY_HEIGHT)?,
            keyboard: Keyboard::new(&sdl_context)?,
        };
        emulator.run(&mut peripherals);
        Ok(())
    }
}
