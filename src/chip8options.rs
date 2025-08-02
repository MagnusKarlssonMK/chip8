//! # Chip8 options
//!
//! Holds a TOML structure for the optional settings of the CHIP-8 program.
//! Will try to read options.toml from the project root directory and deserialize it,
//! if that fails it will use default settings.
use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
pub struct Display {
    pub display_width: u32,
    pub display_height: u32,
    pub scaling: u32,
    pub color_off_rgb: (u8, u8, u8),
    pub color_on_rgb: (u8, u8, u8),
}

impl Default for Display {
    fn default() -> Self {
        Display {
            display_width: 64,
            display_height: 32,
            scaling: 20,
            color_off_rgb: (0, 0, 0),
            color_on_rgb: (255, 255, 255),
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
pub struct Timing {
    pub display_frequency: u32,
    pub cpu_cycles_per_display_tick: u32,
}

impl Default for Timing {
    fn default() -> Self {
        Timing {
            display_frequency: 60,
            cpu_cycles_per_display_tick: 10,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
pub struct Opcode {
    pub shift_ignore_vy: bool,
    pub jump_w_offset_use_vx: bool,
    pub store_load_mem_use_i: bool,
}

impl Default for Opcode {
    fn default() -> Self {
        Opcode {
            shift_ignore_vy: true,
            jump_w_offset_use_vx: false,
            store_load_mem_use_i: false,
        }
    }
}

#[derive(Deserialize, Clone, Copy)]
pub struct Memory {
    pub mem_size: u16,
    pub rom_start: u16,
    pub font_start: u16,
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            mem_size: 4096,
            rom_start: 0x200,
            font_start: 0x50,
        }
    }
}

#[derive(Deserialize, Default, Clone, Copy)]
pub struct Chip8options {
    pub display: Display,
    pub timing: Timing,
    pub opcode: Opcode,
    pub memory: Memory,
}

impl Chip8options {
    pub fn get_options() -> Self {
        let cwd = match std::env::current_dir() {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Failed to get current directory, using default options");
                return Self::default();
            }
        };
        let filename = cwd.join("options.toml");

        let options_file_str = match std::fs::read_to_string(filename) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Failed to read config file, using default options");
                return Self::default();
            }
        };

        let options_file: Self = match toml::from_str(&options_file_str) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("Failed to deserialize config file, using default options");
                Self::default()
            }
        };

        options_file
    }
}
