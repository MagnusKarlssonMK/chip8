use chip8::opcode::*;

#[test]
fn test_tooshortinput() {
    assert!(OpCode::from_bytes(&[0]).is_none());
}

#[test]
fn test_nonexistent() {
    assert!(OpCode::from_bytes(&[0x81, 0x0D]).is_none());
}

#[test]
fn test_clearscreen() {
    if let Some(op) = OpCode::from_bytes(&[0, 0xe0]) {
        match op {
            OpCode::ClearScreen => (),
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_return() {
    if let Some(op) = OpCode::from_bytes(&[0, 0xEE]) {
        match op {
            OpCode::Return => (),
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_jump() {
    if let Some(op) = OpCode::from_bytes(&[0x12, 0x34]) {
        match op {
            OpCode::Jump(v) => {
                assert!(v == 0x234);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_call() {
    if let Some(op) = OpCode::from_bytes(&[0x23, 0x45]) {
        match op {
            OpCode::Call(v) => {
                assert!(v == 0x345);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_set() {
    if let Some(op) = OpCode::from_bytes(&[0x63, 0x45]) {
        match op {
            OpCode::Set { vx, val } => {
                assert!(vx == 3 && val == 0x45);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_add() {
    if let Some(op) = OpCode::from_bytes(&[0x73, 0x45]) {
        match op {
            OpCode::Add { vx, val } => {
                assert!(vx == 3 && val == 0x45);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_skipifvxeq() {
    if let Some(op) = OpCode::from_bytes(&[0x31, 0x24]) {
        match op {
            OpCode::SkipIfVxEq { vx, val } => {
                assert!(vx == 1 && val == 0x24);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_skipifvxneq() {
    if let Some(op) = OpCode::from_bytes(&[0x41, 0x25]) {
        match op {
            OpCode::SkipIfVxNeq { vx, val } => {
                assert!(vx == 1 && val == 0x25);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_skipifvxeqvy() {
    if let Some(op) = OpCode::from_bytes(&[0x51, 0x20]) {
        match op {
            OpCode::SkipIfVxEqVy { vx, vy } => {
                assert!(vx == 1 && vy == 2);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_op5nonexistent() {
    for i in 1..0xF {
        assert!(OpCode::from_bytes(&[0x51, i]).is_none());
    }
}

#[test]
fn test_skipifvxneqvy() {
    if let Some(op) = OpCode::from_bytes(&[0x91, 0x20]) {
        match op {
            OpCode::SkipIfVxNeqVy { vx, vy } => {
                assert!(vx == 1 && vy == 2);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_op9nonexistent() {
    for i in 1..0xF {
        assert!(OpCode::from_bytes(&[0x91, i]).is_none());
    }
}

#[test]
fn test_setvxtovy() {
    if let Some(op) = OpCode::from_bytes(&[0x81, 0x20]) {
        match op {
            OpCode::SetVxToVy { vx, vy } => {
                assert!(vx == 1 && vy == 2);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_binaryor() {
    if let Some(op) = OpCode::from_bytes(&[0x84, 0x51]) {
        match op {
            OpCode::BinaryOr { vx, vy } => {
                assert!(vx == 4 && vy == 5);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_binaryand() {
    if let Some(op) = OpCode::from_bytes(&[0x84, 0x52]) {
        match op {
            OpCode::BinaryAnd { vx, vy } => {
                assert!(vx == 4 && vy == 5);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_logicalxor() {
    if let Some(op) = OpCode::from_bytes(&[0x84, 0x53]) {
        match op {
            OpCode::LogicalXor { vx, vy } => {
                assert!(vx == 4 && vy == 5);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_addvytovx() {
    if let Some(op) = OpCode::from_bytes(&[0x84, 0x54]) {
        match op {
            OpCode::AddVyToVx { vx, vy } => {
                assert!(vx == 4 && vy == 5);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_subvxvytovx() {
    if let Some(op) = OpCode::from_bytes(&[0x84, 0x65]) {
        match op {
            OpCode::SubVxVyToVx { vx, vy } => {
                assert!(vx == 4 && vy == 6);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_subvyvxtovx() {
    if let Some(op) = OpCode::from_bytes(&[0x84, 0x67]) {
        match op {
            OpCode::SubVyVxToVx { vx, vy } => {
                assert!(vx == 4 && vy == 6);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_shift() {
    if let Some(op) = OpCode::from_bytes(&[0x84, 0x16]) {
        match op {
            OpCode::Shift { vx, vy, left_shift } => {
                assert!(vx == 4 && vy == 1 && !left_shift);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }

    if let Some(op) = OpCode::from_bytes(&[0x84, 0x1E]) {
        match op {
            OpCode::Shift { vx, vy, left_shift } => {
                assert!(vx == 4 && vy == 1 && left_shift);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_op8nonexistent() {
    for i in [8, 9, 0xA, 0xB, 0xC, 0xD, 0xF] {
        assert!(OpCode::from_bytes(&[0x84, i]).is_none());
    }
}

#[test]
fn test_setindex() {
    if let Some(op) = OpCode::from_bytes(&[0xA4, 0x67]) {
        match op {
            OpCode::SetIndex(val) => {
                assert!(val == 0x467);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_jumpwithoffset() {
    if let Some(op) = OpCode::from_bytes(&[0xB4, 0x67]) {
        match op {
            OpCode::JumpWithOffset { vx, val } => {
                assert!(vx == 4 && val == 0x467);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_random() {
    if let Some(op) = OpCode::from_bytes(&[0xC4, 0x67]) {
        match op {
            OpCode::Random { vx, val } => {
                assert!(vx == 4 && val == 0x67);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_display() {
    if let Some(op) = OpCode::from_bytes(&[0xD4, 0x67]) {
        match op {
            OpCode::Display { vx, vy, val } => {
                assert!(vx == 4 && vy == 6 && val == 7);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_skipifkeypressed() {
    if let Some(op) = OpCode::from_bytes(&[0xE4, 0x9E]) {
        match op {
            OpCode::SkipIfKeyPressed { vx } => {
                assert!(vx == 4);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_skipifkeynotpressed() {
    if let Some(op) = OpCode::from_bytes(&[0xEF, 0xA1]) {
        match op {
            OpCode::SkipIfKeyNotPressed { vx } => {
                assert!(vx == 0xF);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_openonexistent() {
    assert!(OpCode::from_bytes(&[0xE1, 0x9D]).is_none());
    assert!(OpCode::from_bytes(&[0xE1, 0xA2]).is_none());
}

#[test]
fn test_setvxtodelaytimer() {
    if let Some(op) = OpCode::from_bytes(&[0xF1, 0x07]) {
        match op {
            OpCode::SetVxToDelayTimer { vx } => {
                assert!(vx == 1);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_setdelaytimertovx() {
    if let Some(op) = OpCode::from_bytes(&[0xF1, 0x15]) {
        match op {
            OpCode::SetDelayTimerToVx { vx } => {
                assert!(vx == 1);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_setsoundtimertovx() {
    if let Some(op) = OpCode::from_bytes(&[0xF1, 0x18]) {
        match op {
            OpCode::SetSoundTimerToVx { vx } => {
                assert!(vx == 1);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_addtoindex() {
    if let Some(op) = OpCode::from_bytes(&[0xF3, 0x1E]) {
        match op {
            OpCode::AddToIndex { vx } => {
                assert!(vx == 3);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_getkey() {
    if let Some(op) = OpCode::from_bytes(&[0xF3, 0x0A]) {
        match op {
            OpCode::GetKey { vx } => {
                assert!(vx == 3);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_fontchar() {
    if let Some(op) = OpCode::from_bytes(&[0xF3, 0x29]) {
        match op {
            OpCode::FontCharacter { vx } => {
                assert!(vx == 3);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_binarycodeddecconv() {
    if let Some(op) = OpCode::from_bytes(&[0xF4, 0x33]) {
        match op {
            OpCode::BinaryCodedDecimalConversion { vx } => {
                assert!(vx == 4);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_storememory() {
    if let Some(op) = OpCode::from_bytes(&[0xF3, 0x55]) {
        match op {
            OpCode::StoreMemory { vx } => {
                assert!(vx == 3);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}

#[test]
fn test_loadmemory() {
    if let Some(op) = OpCode::from_bytes(&[0xF3, 0x65]) {
        match op {
            OpCode::LoadMemory { vx } => {
                assert!(vx == 3);
            }
            _ => assert!(false),
        }
    } else {
        assert!(false);
    }
}
