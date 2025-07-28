//! # Emulator
//!
//! The main part of the CHIP-8 emulator module
use crate::opcode::OpCode;

const MEM_SIZE: usize = 4096;
const ROM_START: usize = 0x200;
const FONT_START: usize = 0x50;

const FONTS: [u8; 16 * 5] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

/// Trait used by emulator to trigger a screen update, providing the current display
/// bits in a vector.
///
/// TBD - Maybe figure out a way to support different resolutions for future extensions? Add it as inputs here, or assume that the outer module can access it somehow?
pub trait Screen {
    fn update_screen(&self, display_output: &[bool]);
}

/// Contains the data for the emulator
pub struct Emulator {
    memory: Vec<u8>,
    display_output: Vec<bool>,
    pc: u16,
    reg_i: u16,
    stack: Vec<u16>,
    // Missing vx registers???
    delay_timer: u8,
    sound_timer: u8,
}

impl Emulator {
    /// Creates a new Emulator instance. Takes the rom vector as input.
    pub fn new(rom: &[u8]) -> Self {
        let mut memory = vec![0; MEM_SIZE];
        for (i, n) in FONTS.iter().enumerate() {
            memory[i + FONT_START] = *n;
        }
        for (i, n) in rom.iter().enumerate() {
            memory[i + ROM_START] = *n;
        }

        // Temporary debug - remove me
        for (i, _) in rom.iter().step_by(2).enumerate() {
            println!("{:?}", OpCode::from_bytes(&rom[i..]));
        }
        Self {
            memory,
            display_output: vec![false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            pc: 0x200,
            reg_i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    /// Starts running the emulator until the program is halted.
    pub fn run<T: Screen>(&mut self, screen_handle: &T) {
        if self.pc > 0 {
            // Just a temporary placeholder hack to make use of the function attributes
            screen_handle.update_screen(&self.display_output);
        }
    }
}
