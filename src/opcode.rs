//! # Op code representation
//!
//! Contains all op codes for CHIP-8 as an enum, and a decoding function.

/// Enum to represent all possible CHIP-8 OP codes
#[derive(Debug)]
pub enum OpCode {
    ClearScreen,
    Return,
    Jump(u16),
    Call(u16),
    Set { vx: u8, val: u8 },
    Add { vx: u8, val: u8 },
    SkipIfVxEq { vx: u8, val: u8 },
    SkipIfVxNeq { vx: u8, val: u8 },
    SkipIfVxEqVy { vx: u8, vy: u8 },
    SkipIfVxNeqVy { vx: u8, vy: u8 },
    SetVxToVy { vx: u8, vy: u8 },
    BinaryOr { vx: u8, vy: u8 },
    BinaryAnd { vx: u8, vy: u8 },
    LogicalXor { vx: u8, vy: u8 },
    AddVyToVx { vx: u8, vy: u8 },
    SubVxVyToVx { vx: u8, vy: u8 },
    SubVyVxToVx { vx: u8, vy: u8 },
    Shift { vx: u8, vy: u8, left_shift: bool },
    SetIndex(u16),
    JumpWithOffset { vx: u8, val: u16 },
    Random { vx: u8, val: u8 },
    Display { vx: u8, vy: u8, val: u8 },
    SkipIfKeyPressed { vx: u8 },
    SkipIfKeyNotPressed { vx: u8 },
    SetVxToDelayTimer { vx: u8 },
    SetDelayTimerToVx { vx: u8 },
    SetSoundTimerToVx { vx: u8 },
    AddToIndex { vx: u8 },
    GetKey { vx: u8 },
    FontCharacter { vx: u8 },
    BinaryCodedDecimalConversion { vx: u8 },
    StoreMemory { vx: u8 },
    LoadMemory { vx: u8 },
}

impl OpCode {
    /// Takes a u8 slice as input and converts it to an op code. Will return None if length
    /// of input is less than 2, or if the input doesn't match any supported op code.
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 2 {
            return None;
        }
        let vx = bytes[0] & 0xF;
        let vy = (bytes[1] & 0xF0) >> 4;
        match bytes[0] >> 4 {
            0 => match ((bytes[0] as u16) << 8) + bytes[1] as u16 {
                0x00E0 => Some(Self::ClearScreen),
                0x00EE => Some(Self::Return),
                _ => None,
            },
            1 => Some(Self::Jump(((vx as u16) << 8) + bytes[1] as u16)),
            2 => Some(Self::Call(((vx as u16) << 8) + bytes[1] as u16)),
            3 => Some(Self::SkipIfVxEq { vx, val: bytes[1] }),
            4 => Some(Self::SkipIfVxNeq { vx, val: bytes[1] }),
            5 => {
                if (bytes[1] & 0xF) == 0 {
                    Some(Self::SkipIfVxEqVy { vx, vy })
                } else {
                    None
                }
            }
            6 => Some(Self::Set { vx, val: bytes[1] }),
            7 => Some(Self::Add { vx, val: bytes[1] }),
            8 => match bytes[1] & 0xF {
                0 => Some(Self::SetVxToVy { vx, vy }),
                1 => Some(Self::BinaryOr { vx, vy }),
                2 => Some(Self::BinaryAnd { vx, vy }),
                3 => Some(Self::LogicalXor { vx, vy }),
                4 => Some(Self::AddVyToVx { vx, vy }),
                5 => Some(Self::SubVxVyToVx { vx, vy }),
                6 => Some(Self::Shift {
                    vx,
                    vy,
                    left_shift: false,
                }),
                7 => Some(Self::SubVyVxToVx { vx, vy }),
                0xE => Some(Self::Shift {
                    vx,
                    vy,
                    left_shift: true,
                }),
                _ => None,
            },
            9 => {
                if (bytes[1] & 0xF) == 0 {
                    Some(Self::SkipIfVxNeqVy { vx, vy })
                } else {
                    None
                }
            }
            0xA => Some(Self::SetIndex(((vx as u16) << 8) + bytes[1] as u16)),
            0xB => Some(Self::JumpWithOffset {
                vx,
                val: (((vx as u16) << 8) + bytes[1] as u16),
            }),
            0xC => Some(Self::Random { vx, val: bytes[1] }),
            0xD => Some(Self::Display {
                vx,
                vy,
                val: bytes[1] & 0xF,
            }),
            0xE => match bytes[1] {
                0x9E => Some(Self::SkipIfKeyPressed { vx }),
                0xA1 => Some(Self::SkipIfKeyNotPressed { vx }),
                _ => None,
            },
            0xf => match bytes[1] {
                0x07 => Some(Self::SetVxToDelayTimer { vx }),
                0x15 => Some(Self::SetDelayTimerToVx { vx }),
                0x18 => Some(Self::SetSoundTimerToVx { vx }),
                0x1E => Some(Self::AddToIndex { vx }),
                0x0A => Some(Self::GetKey { vx }),
                0x29 => Some(Self::FontCharacter { vx }),
                0x33 => Some(Self::BinaryCodedDecimalConversion { vx }),
                0x55 => Some(Self::StoreMemory { vx }),
                0x65 => Some(Self::LoadMemory { vx }),
                _ => None,
            },
            _ => None,
        }
    }
}
