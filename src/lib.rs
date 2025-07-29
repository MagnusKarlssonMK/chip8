use std::{env, error::Error, fs};

use emulator::Emulator;

pub mod emulator;
pub mod opcode;

#[allow(dead_code)]
struct SdlContext {
    gfx_handle: usize,
}

const DISPLAY_WIDTH: u8 = 64;
const DISPLAY_HEIGHT: u8 = 32;

#[allow(dead_code)]
impl emulator::System for SdlContext {
    fn update_screen(&self, _display_output: &[bool]) {
        for (i, v) in _display_output.iter().enumerate() {
            if i % 64 == 0 {
                println!();
            }
            if *v {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
        //println!("{} - {}", self.gfx_handle, display_output[0]);
    }

    fn get_key_event(&self) -> Option<emulator::KeyEvent> {
        None
    }
}

pub struct Config {
    rom_file: String,
    sdl_context: SdlContext,
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

        let sdl_context = SdlContext { gfx_handle: 0 };
        Ok(Config {
            rom_file,
            sdl_context,
        })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        // Read input file
        let cwd = env::current_dir()?;
        let filename = cwd
            .join("rom_files")
            .join(format!("{}.ch8", &self.rom_file));

        let rom = fs::read(filename)?.to_vec();

        //println!("ROM: {rom:?}");

        let mut _em = Emulator::new(&rom, DISPLAY_WIDTH, DISPLAY_HEIGHT);
        _em.run(&self.sdl_context);
        Ok(())
    }
}
