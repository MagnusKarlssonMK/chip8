//! # Keyboard
//!
//! Contains the keyboard module for CHIP-8.

use crate::emulator::KeyEvent;
use sdl2::{EventPump, Sdl, event::Event, keyboard::Keycode};
use std::error::Error;

/// Utility function to map the SDL keycode to the internal CHIP-8 number 0x0-0xF.
fn map_keycode(key: &Keycode) -> Option<u8> {
    match *key {
        Keycode::Num1 => Some(0x1),
        Keycode::Num2 => Some(0x2),
        Keycode::Num3 => Some(0x3),
        Keycode::Num4 => Some(0xC),

        Keycode::Q => Some(0x4),
        Keycode::W => Some(0x5),
        Keycode::E => Some(0x6),
        Keycode::R => Some(0xD),

        Keycode::A => Some(0x7),
        Keycode::S => Some(0x8),
        Keycode::D => Some(0x9),
        Keycode::F => Some(0xE),

        Keycode::Z => Some(0xA),
        Keycode::X => Some(0x0),
        Keycode::C => Some(0xB),
        Keycode::V => Some(0xF),
        _ => None,
    }
}

pub struct Keyboard {
    event_pump: EventPump,
}

impl Keyboard {
    /// Creates a new Keyboard instance
    pub fn new(sdl_context: &Sdl) -> Result<Self, Box<dyn Error>> {
        let event_pump = sdl_context.event_pump()?;
        Ok(Self { event_pump })
    }

    /// Get the next queued up CHIP-8 key. Ignores unmapped events in the queue.
    /// Returns None if nothing is queued.
    pub fn get_chip8_key_events(&mut self) -> Option<KeyEvent> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return Some(KeyEvent::Quit);
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = map_keycode(&key) {
                        return Some(KeyEvent::KeyDown(k));
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = map_keycode(&key) {
                        return Some(KeyEvent::KeyUp(k));
                    }
                }
                _ => (),
            }
        }
        None
    }
}
