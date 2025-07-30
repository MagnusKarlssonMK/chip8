//! # Keyboard
//!
//! Contains the keyboard module for CHIP-8.

use crate::emulator::KeyEvent;
use sdl2::{EventPump, Sdl, event::Event, keyboard::Keycode};
use std::error::Error;

/// Utility function to map the SDL keycode to the internal CHIP-8 number 0x0-0xF.
fn map_keycode(key: &Keycode) -> Option<u8> {
    match *key {
        Keycode::Num1 => Some(0),
        Keycode::Num2 => Some(1),
        Keycode::Num3 => Some(2),
        Keycode::Num4 => Some(3),

        Keycode::Q => Some(4),
        Keycode::W => Some(5),
        Keycode::E => Some(6),
        Keycode::R => Some(7),

        Keycode::A => Some(8),
        Keycode::S => Some(9),
        Keycode::D => Some(10),
        Keycode::F => Some(11),

        Keycode::Z => Some(12),
        Keycode::X => Some(13),
        Keycode::C => Some(14),
        Keycode::V => Some(15),
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
