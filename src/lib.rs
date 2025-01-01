use std::{env, error::Error, fs};

pub struct Config {
    pub rom_file: String,
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

        Ok(Config { rom_file })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        // Read input file
        let cwd = env::current_dir()?;
        let filename = cwd
            .join("rom_files")
            .join(format!("{}.ch8", &self.rom_file));

        let rom = fs::read(filename)?.to_vec();

        println!("ROM: {:?}", rom);

        Ok(())
    }
}
