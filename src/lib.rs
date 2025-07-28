use std::{env, error::Error, fs};

use emulator::Emulator;

pub mod emulator;
pub mod opcode;

struct SdlContext {
    gfx_handle: usize,
}

impl emulator::Screen for SdlContext {
    fn update_screen(&self, display_output: &[bool]) {
        println!("{} - {}", self.gfx_handle, display_output[0]);
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
                )
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

        println!("ROM: {rom:?}");

        let mut _em = Emulator::new(&rom);
        _em.run(&self.sdl_context);
        Ok(())
    }
}
