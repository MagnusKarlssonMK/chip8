//! # Emulator
//!
//! The main part of the CHIP-8 emulator module
use crate::chip8options::Chip8options;
use crate::opcode::OpCode;
use rand::Rng;
use std::time::Duration;

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

pub enum KeyEvent {
    KeyDown(u8),
    KeyUp(u8),
    Quit,
}

/// Trait used by emulator to trigger a screen update, providing the current display
/// bits in a vector.
pub trait System {
    fn update_screen(&mut self, display_output: &[bool]);

    fn get_key_event(&mut self) -> Option<KeyEvent>;
}

/// Contains the data for the emulator
pub struct Emulator {
    memory: Vec<u8>,
    display_output: Vec<bool>,
    display_width: u8,
    display_height: u8,
    pc: u16,
    reg_i: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    reg_vx: [u8; 16],
    keypad: [bool; 16],
    options: Chip8options,
    rng: rand::rngs::ThreadRng,
    display_updated: bool,
}

impl Emulator {
    /// Creates a new Emulator instance. Takes the rom vector as input and the configuration options.
    pub fn new(rom: &[u8], options: &Chip8options) -> Self {
        let mut memory = vec![0; options.memory.mem_size as usize];
        for (i, n) in FONTS.iter().enumerate() {
            memory[i + options.memory.font_start as usize] = *n;
        }
        for (i, n) in rom.iter().enumerate() {
            memory[i + options.memory.rom_start as usize] = *n;
        }

        Self {
            memory,
            display_output: vec![
                false;
                options.display.display_width as usize
                    * options.display.display_height as usize
            ],
            display_width: options.display.display_width as u8,
            display_height: options.display.display_height as u8,
            pc: 0x200,
            reg_i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            reg_vx: [0; 16],
            keypad: [false; 16],
            options: *options,
            rng: rand::rng(),
            display_updated: false,
        }
    }

    /// Starts running the emulator until the program is halted.
    pub fn run<T: System>(&mut self, system_handle: &mut T) {
        'running: loop {
            // Decrement timers
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                // TBD - no actual sound (or representation of sound) yet
                self.sound_timer -= 1;
            }

            let mut wait_for_display_interrupt = false;
            for cpu_cycle in 0..self.options.timing.cpu_cycles_per_display_tick {
                // Empty the key events
                while let Some(k) = system_handle.get_key_event() {
                    match k {
                        KeyEvent::Quit => {
                            break 'running;
                        }
                        KeyEvent::KeyDown(k) => {
                            self.keypad[k as usize] = true;
                        }
                        KeyEvent::KeyUp(k) => {
                            self.keypad[k as usize] = false;
                        }
                    }
                }

                // Process CPU op codes
                let opcode = OpCode::from_bytes(&self.memory[self.pc as usize..]);
                self.pc += 2;
                if let Some(op) = opcode {
                    match op {
                        OpCode::Display { .. } => {
                            if cpu_cycle > 0 && wait_for_display_interrupt {
                                self.pc -= 2;
                                break;
                            }
                        }
                        _ => {
                            wait_for_display_interrupt = true;
                        }
                    }
                    self.execute_opcode(op);
                } else {
                    eprintln!("Warning: Failed to decode op code");
                }
            }
            if self.display_updated {
                system_handle.update_screen(&self.display_output);
                self.display_updated = false;
            }

            // If we want to be really picky about time, maybe consider subtracting the execution time of this loop cycle
            std::thread::sleep(Duration::new(
                0,
                1_000_000_000u32 / self.options.timing.display_frequency,
            ));
        }
    }

    /// Performs the OP code operation on the emulator
    fn execute_opcode(&mut self, opcode: OpCode) {
        match opcode {
            OpCode::ClearScreen => {
                for v in self.display_output.iter_mut() {
                    *v = false;
                }
                self.display_updated = true;
            }
            OpCode::Jump(v) => {
                self.pc = v;
            }
            OpCode::Call(v) => {
                self.stack.push(self.pc);
                self.pc = v;
            }
            OpCode::Return => {
                if let Some(r) = self.stack.pop() {
                    self.pc = r;
                } else {
                    eprintln!("Warning: Return called with empty stack.")
                }
            }
            OpCode::Set { vx, val } => {
                self.reg_vx[vx as usize] = val;
            }
            OpCode::Add { vx, val } => {
                self.reg_vx[vx as usize] = self.reg_vx[vx as usize].wrapping_add(val);
            }
            OpCode::SkipIfVxEq { vx, val } => {
                if self.reg_vx[vx as usize] == val {
                    self.pc += 2;
                }
            }
            OpCode::SkipIfVxNeq { vx, val } => {
                if self.reg_vx[vx as usize] != val {
                    self.pc += 2;
                }
            }
            OpCode::SkipIfVxEqVy { vx, vy } => {
                if self.reg_vx[vx as usize] == self.reg_vx[vy as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SkipIfVxNeqVy { vx, vy } => {
                if self.reg_vx[vx as usize] != self.reg_vx[vy as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SetVxToVy { vx, vy } => {
                self.reg_vx[vx as usize] = self.reg_vx[vy as usize];
                self.reg_vx[0xF] = 0;
            }
            OpCode::BinaryOr { vx, vy } => {
                self.reg_vx[vx as usize] |= self.reg_vx[vy as usize];
                self.reg_vx[0xF] = 0;
            }
            OpCode::BinaryAnd { vx, vy } => {
                self.reg_vx[vx as usize] &= self.reg_vx[vy as usize];
                self.reg_vx[0xF] = 0;
            }
            OpCode::LogicalXor { vx, vy } => {
                self.reg_vx[vx as usize] ^= self.reg_vx[vy as usize];
                self.reg_vx[0xF] = 0;
            }
            OpCode::AddVyToVx { vx, vy } => {
                let old_vx = self.reg_vx[vx as usize];
                self.reg_vx[vx as usize] = old_vx.wrapping_add(self.reg_vx[vy as usize]);
                self.reg_vx[0xF] = if self.reg_vx[vx as usize] < old_vx {
                    1
                } else {
                    0
                };
            }
            OpCode::SubVxVyToVx { vx, vy } => {
                let old_vx = self.reg_vx[vx as usize];
                self.reg_vx[vx as usize] = old_vx.wrapping_sub(self.reg_vx[vy as usize]);
                self.reg_vx[0xF] = if old_vx >= self.reg_vx[vy as usize] {
                    1
                } else {
                    0
                };
            }
            OpCode::SubVyVxToVx { vx, vy } => {
                let old_vx = self.reg_vx[vx as usize];
                self.reg_vx[vx as usize] = self.reg_vx[vy as usize].wrapping_sub(old_vx);
                self.reg_vx[0xF] = if self.reg_vx[vy as usize] >= old_vx {
                    1
                } else {
                    0
                };
            }
            OpCode::Shift { vx, vy, left_shift } => {
                if !self.options.opcode.shift_ignore_vy {
                    self.reg_vx[vx as usize] = self.reg_vx[vy as usize];
                }
                let old_vx = self.reg_vx[vx as usize];
                if left_shift {
                    self.reg_vx[vx as usize] = (self.reg_vx[vx as usize] & 0x7F) << 1;
                    self.reg_vx[0xF] = old_vx >> 7;
                } else {
                    self.reg_vx[vx as usize] >>= 1;
                    self.reg_vx[0xF] = old_vx & 0x1;
                }
            }
            OpCode::SetIndex(val) => {
                self.reg_i = val;
            }
            OpCode::JumpWithOffset { vx, val } => {
                self.pc = if self.options.opcode.jump_w_offset_use_vx {
                    val + self.reg_vx[vx as usize] as u16
                } else {
                    val + self.reg_vx[0] as u16
                };
            }
            OpCode::Random { vx, val } => {
                let rnd: u8 = self.rng.random();
                self.reg_vx[vx as usize] = rnd & val;
            }
            OpCode::Display { vx, vy, val } => {
                let x_start = self.reg_vx[vx as usize] % self.display_width;
                let y_start = self.reg_vx[vy as usize] % self.display_height;
                let x_stop = (x_start + 8).min(self.display_width);
                let y_stop = (y_start + val).min(self.display_height);
                self.reg_vx[0xF] = 0;
                for (n, y) in (y_start..y_stop).enumerate() {
                    let sprite = self.memory[self.reg_i as usize + n];
                    for (i, x) in (x_start..x_stop).enumerate() {
                        let idx = x as usize + ((y as usize) * self.display_width as usize);
                        let old_pixel = self.display_output[idx];
                        let new_pixel = (sprite & (0x80 >> i)) > 0;
                        self.display_output[idx] = old_pixel ^ new_pixel;
                        if old_pixel && new_pixel {
                            self.reg_vx[0xF] = 1;
                        }
                    }
                }
                self.display_updated = true;
            }
            OpCode::SkipIfKeyPressed { vx } => {
                if self.keypad[self.reg_vx[vx as usize & 0xF] as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SkipIfKeyNotPressed { vx } => {
                if !self.keypad[self.reg_vx[vx as usize & 0xF] as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SetVxToDelayTimer { vx } => {
                self.reg_vx[vx as usize] = self.delay_timer;
            }
            OpCode::SetDelayTimerToVx { vx } => {
                self.delay_timer = self.reg_vx[vx as usize];
            }
            OpCode::SetSoundTimerToVx { vx } => {
                self.sound_timer = self.reg_vx[vx as usize];
            }
            OpCode::AddToIndex { vx } => {
                self.reg_i += self.reg_vx[vx as usize] as u16;
                // Overflow handling
                if self.reg_i >= self.options.memory.mem_size {
                    self.reg_i %= self.options.memory.mem_size;
                    self.reg_vx[0xF] = 1;
                }
            }
            OpCode::GetKey { vx } => {
                // Simplified implementation - Accept any key currently pressed, lowest key prioritized if multiple keys pressed.
                let mut keypressed = false;
                for (key, v) in self.keypad.iter().enumerate() {
                    if *v {
                        self.reg_vx[vx as usize] = key as u8;
                        keypressed = true;
                        break;
                    }
                }
                if !keypressed {
                    self.pc -= 2;
                }
            }
            OpCode::FontCharacter { vx } => {
                // Masking the font index in VX just in case its value is too big.
                self.reg_i =
                    self.options.memory.font_start + (5 * (self.reg_vx[vx as usize] & 0xF) as u16);
            }
            OpCode::BinaryCodedDecimalConversion { vx } => {
                self.memory[self.reg_i as usize] = self.reg_vx[vx as usize] / 100;
                self.memory[self.reg_i as usize + 1] = (self.reg_vx[vx as usize] % 100) / 10;
                self.memory[self.reg_i as usize + 2] = self.reg_vx[vx as usize] % 10;
            }
            OpCode::StoreMemory { vx } => {
                for x in 0..vx + 1 {
                    self.memory[self.reg_i as usize + x as usize] = self.reg_vx[x as usize];
                }
                if self.options.opcode.store_load_mem_use_i {
                    self.reg_i += vx as u16 + 1;
                }
            }
            OpCode::LoadMemory { vx } => {
                for x in 0..vx + 1 {
                    self.reg_vx[x as usize] = self.memory[self.reg_i as usize + x as usize];
                }
                if self.options.opcode.store_load_mem_use_i {
                    self.reg_i += vx as u16 + 1;
                }
            }
        }
    }
}

#[path = "unittest/test_emulator.rs"]
#[cfg(test)]
mod test_emulator;
