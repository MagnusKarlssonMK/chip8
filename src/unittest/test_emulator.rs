use super::*;

#[test]
fn test_jump() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    let jump_pos = 0x250;
    test_emulator.execute_opcode(OpCode::Jump(jump_pos));
    assert!(test_emulator.pc == jump_pos);
}

#[test]
fn test_call_return() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    let start_pos = test_emulator.pc;
    let call_pos_1 = 0x250;
    let call_pos_2 = 0x260;
    test_emulator.execute_opcode(OpCode::Call(call_pos_1));
    assert!(
        test_emulator.pc == call_pos_1
            && test_emulator.stack.len() == 1
            && test_emulator.stack[0] == start_pos
    );
    test_emulator.execute_opcode(OpCode::Call(call_pos_2));
    assert!(
        test_emulator.pc == call_pos_2
            && test_emulator.stack.len() == 2
            && test_emulator.stack[1] == call_pos_1
    );
    test_emulator.execute_opcode(OpCode::Return);
    assert!(
        test_emulator.pc == call_pos_1
            && test_emulator.stack.len() == 1
            && test_emulator.stack[0] == start_pos
    );
    test_emulator.execute_opcode(OpCode::Return);
    assert!(test_emulator.pc == start_pos && test_emulator.stack.is_empty());
}

#[test]
fn test_set_add() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.execute_opcode(OpCode::Set { vx: 1, val: 3 });
    test_emulator.execute_opcode(OpCode::Set { vx: 2, val: 8 });
    assert!(test_emulator.reg_vx[1] == 3 && test_emulator.reg_vx[2] == 8);
    test_emulator.execute_opcode(OpCode::Add { vx: 1, val: 1 });
    assert!(test_emulator.reg_vx[1] == 4 && test_emulator.reg_vx[0xF] == 0);
    test_emulator.execute_opcode(OpCode::Add { vx: 2, val: 247 });
    assert!(test_emulator.reg_vx[2] == 255 && test_emulator.reg_vx[0xF] == 0);
    test_emulator.execute_opcode(OpCode::Add { vx: 2, val: 2 });
    assert!(test_emulator.reg_vx[2] == 1 && test_emulator.reg_vx[0xF] == 0);
}

#[test]
fn test_skip_if_vx() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    let mut prev_pos = test_emulator.pc;
    test_emulator.execute_opcode(OpCode::SkipIfVxEq { vx: 1, val: 1 });
    assert!(test_emulator.pc == prev_pos);
    test_emulator.execute_opcode(OpCode::SkipIfVxEq { vx: 1, val: 0 });
    assert!(test_emulator.pc == prev_pos + 2);
    prev_pos = test_emulator.pc;
    test_emulator.execute_opcode(OpCode::SkipIfVxNeq { vx: 1, val: 0 });
    assert!(test_emulator.pc == prev_pos);
    test_emulator.execute_opcode(OpCode::SkipIfVxNeq { vx: 1, val: 1 });
    assert!(test_emulator.pc == prev_pos + 2);
}

#[test]
fn test_set_vxtovy() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.reg_vx[3] = 7;
    test_emulator.reg_vx[0xf] = 1;
    test_emulator.execute_opcode(OpCode::SetVxToVy { vx: 2, vy: 3 });
    assert!(test_emulator.reg_vx[2] == 7 && test_emulator.reg_vx[3] == 7);
    assert!(test_emulator.reg_vx[0xf] == 0);
}

#[test]
fn test_arithmetic_logical() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.reg_vx[2] = 4;
    test_emulator.reg_vx[3] = 1;
    test_emulator.reg_vx[0xf] = 1;
    test_emulator.execute_opcode(OpCode::SetVxToVy { vx: 1, vy: 2 });
    assert!(test_emulator.reg_vx[1] == 4 && test_emulator.reg_vx[2] == 4);
    assert!(test_emulator.reg_vx[0xf] == 0);
    test_emulator.reg_vx[0xf] = 1;
    test_emulator.execute_opcode(OpCode::BinaryOr { vx: 1, vy: 3 });
    assert!(test_emulator.reg_vx[1] == 5 && test_emulator.reg_vx[3] == 1);
    assert!(test_emulator.reg_vx[0xf] == 0);
    test_emulator.reg_vx[0xf] = 1;
    test_emulator.execute_opcode(OpCode::BinaryAnd { vx: 1, vy: 2 });
    assert!(test_emulator.reg_vx[1] == 4 && test_emulator.reg_vx[2] == 4);
    assert!(test_emulator.reg_vx[0xf] == 0);
    test_emulator.reg_vx[1] = 5;
    test_emulator.reg_vx[0xf] = 1;
    test_emulator.execute_opcode(OpCode::LogicalXor { vx: 1, vy: 3 });
    assert!(test_emulator.reg_vx[1] == 4 && test_emulator.reg_vx[3] == 1);
    assert!(test_emulator.reg_vx[0xf] == 0);
    // Add with no overflow
    test_emulator.reg_vx[2] = 251;
    test_emulator.reg_vx[0xf] = 1;
    test_emulator.execute_opcode(OpCode::AddVyToVx { vx: 1, vy: 2 });
    assert!(test_emulator.reg_vx[1] == 255 && test_emulator.reg_vx[2] == 251);
    assert!(test_emulator.reg_vx[0xF] == 0);
    // Add with overflow
    test_emulator.execute_opcode(OpCode::AddVyToVx { vx: 1, vy: 3 });
    assert!(test_emulator.reg_vx[1] == 0 && test_emulator.reg_vx[3] == 1);
    assert!(test_emulator.reg_vx[0xF] == 1);
    // Add zero
    test_emulator.execute_opcode(OpCode::AddVyToVx { vx: 2, vy: 1 });
    assert!(test_emulator.reg_vx[2] == 251 && test_emulator.reg_vx[1] == 0);
    assert!(test_emulator.reg_vx[0xF] == 0);
    // Sub X - Y > 0
    test_emulator.execute_opcode(OpCode::SubVxVyToVx { vx: 2, vy: 3 });
    assert!(test_emulator.reg_vx[2] == 250 && test_emulator.reg_vx[3] == 1);
    assert!(test_emulator.reg_vx[0xF] == 1);
    // Sub with zero
    test_emulator.execute_opcode(OpCode::SubVxVyToVx { vx: 2, vy: 1 });
    assert!(test_emulator.reg_vx[2] == 250 && test_emulator.reg_vx[1] == 0);
    assert!(test_emulator.reg_vx[0xF] == 1);
    // Sub to zero
    test_emulator.reg_vx[0xF] = 0;
    test_emulator.reg_vx[4] = 1;
    test_emulator.execute_opcode(OpCode::SubVxVyToVx { vx: 4, vy: 3 });
    assert!(test_emulator.reg_vx[4] == 0 && test_emulator.reg_vx[3] == 1);
    assert!(test_emulator.reg_vx[0xF] == 1);
    // Sub X - Y < 0
    test_emulator.execute_opcode(OpCode::SubVxVyToVx { vx: 1, vy: 3 });
    assert!(test_emulator.reg_vx[1] == 255 && test_emulator.reg_vx[3] == 1);
    assert!(test_emulator.reg_vx[0xF] == 0);
    // Sub Y - X > 0
    test_emulator.execute_opcode(OpCode::SubVyVxToVx { vx: 2, vy: 1 });
    assert!(test_emulator.reg_vx[2] == 5 && test_emulator.reg_vx[1] == 255);
    assert!(test_emulator.reg_vx[0xF] == 1);
    // Sub Y - X > 0
    test_emulator.execute_opcode(OpCode::SubVyVxToVx { vx: 2, vy: 3 });
    assert!(test_emulator.reg_vx[2] == 252 && test_emulator.reg_vx[3] == 1);
    assert!(test_emulator.reg_vx[0xF] == 0);
    // Sub with zero
    test_emulator.reg_vx[0xF] = 0;
    test_emulator.execute_opcode(OpCode::SubVxVyToVx { vx: 2, vy: 4 });
    assert!(test_emulator.reg_vx[2] == 252 && test_emulator.reg_vx[4] == 0);
    assert!(test_emulator.reg_vx[0xF] == 1);
    // Sub to zero
    test_emulator.reg_vx[0xF] = 0;
    test_emulator.reg_vx[4] = 1;
    test_emulator.execute_opcode(OpCode::SubVxVyToVx { vx: 4, vy: 3 });
    assert!(test_emulator.reg_vx[4] == 0 && test_emulator.reg_vx[3] == 1);
    assert!(test_emulator.reg_vx[0xF] == 1);
}

#[test]
fn test_shift() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    // --- Option 1 ---
    test_emulator.options.opcode.shift_ignore_vy = false;
    // Right shift, shifted out == 0
    test_emulator.reg_vx[1] = 7;
    test_emulator.reg_vx[2] = 2;
    test_emulator.reg_vx[0xF] = 1;
    test_emulator.execute_opcode(OpCode::Shift {
        vx: 1,
        vy: 2,
        left_shift: false,
    });
    assert!(test_emulator.reg_vx[1] == 1 && test_emulator.reg_vx[2] == 2);
    assert!(test_emulator.reg_vx[0xF] == 0);
    // Right shift, shifted out == 1
    test_emulator.reg_vx[2] = 3;
    test_emulator.execute_opcode(OpCode::Shift {
        vx: 1,
        vy: 2,
        left_shift: false,
    });
    assert!(test_emulator.reg_vx[1] == 1 && test_emulator.reg_vx[2] == 3);
    assert!(test_emulator.reg_vx[0xF] == 1);
    // Left shift, shifted out == 0
    test_emulator.reg_vx[2] = 0x60;
    test_emulator.execute_opcode(OpCode::Shift {
        vx: 1,
        vy: 2,
        left_shift: true,
    });
    assert!(test_emulator.reg_vx[1] == 0xC0 && test_emulator.reg_vx[2] == 0x60);
    assert!(test_emulator.reg_vx[0xF] == 0);
    // Left shift, shifted out == 1
    test_emulator.reg_vx[1] = 1;
    test_emulator.reg_vx[2] = 0xC0;
    test_emulator.execute_opcode(OpCode::Shift {
        vx: 1,
        vy: 2,
        left_shift: true,
    });
    assert!(test_emulator.reg_vx[1] == 0x80 && test_emulator.reg_vx[2] == 0xC0);
    assert!(test_emulator.reg_vx[0xF] == 1);

    // --- Option 2 ---
    test_emulator.options.opcode.shift_ignore_vy = true;

    // Right shift, shifted out == 0
    test_emulator.reg_vx[1] = 6;
    test_emulator.reg_vx[2] = 2;
    test_emulator.execute_opcode(OpCode::Shift {
        vx: 1,
        vy: 2,
        left_shift: false,
    });
    assert!(test_emulator.reg_vx[1] == 3 && test_emulator.reg_vx[2] == 2);
    assert!(test_emulator.reg_vx[0xF] == 0);
    // Right shift, shifted out == 1
    test_emulator.execute_opcode(OpCode::Shift {
        vx: 1,
        vy: 2,
        left_shift: false,
    });
    assert!(test_emulator.reg_vx[1] == 1 && test_emulator.reg_vx[2] == 2);
    assert!(test_emulator.reg_vx[0xF] == 1);
    // Left shift, shifted out == 0
    test_emulator.reg_vx[1] = 0x60;
    test_emulator.execute_opcode(OpCode::Shift {
        vx: 1,
        vy: 2,
        left_shift: true,
    });
    assert!(test_emulator.reg_vx[1] == 0xC0 && test_emulator.reg_vx[2] == 2);
    assert!(test_emulator.reg_vx[0xF] == 0);
    // Left shift, shifted out == 1
    test_emulator.execute_opcode(OpCode::Shift {
        vx: 1,
        vy: 2,
        left_shift: true,
    });
    assert!(test_emulator.reg_vx[1] == 0x80 && test_emulator.reg_vx[2] == 2);
    assert!(test_emulator.reg_vx[0xF] == 1);
}

#[test]
fn test_setindex() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.execute_opcode(OpCode::SetIndex(0x350));
    assert!(test_emulator.reg_i == 0x350);
}

#[test]
fn test_jumpwithoffset() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.options.opcode.jump_w_offset_use_vx = false;
    test_emulator.reg_vx[0] = 6;
    test_emulator.reg_vx[1] = 3;
    test_emulator.execute_opcode(OpCode::JumpWithOffset { vx: 1, val: 0x152 });
    assert!(test_emulator.pc == 0x158);

    test_emulator.options.opcode.jump_w_offset_use_vx = true;
    test_emulator.execute_opcode(OpCode::JumpWithOffset { vx: 1, val: 0x152 });
    assert!(test_emulator.pc == 0x155);
    test_emulator.reg_vx[0xE] = 2;
    test_emulator.execute_opcode(OpCode::JumpWithOffset {
        vx: 0xE,
        val: 0xE52,
    });
    assert!(test_emulator.pc == 0xE54);
}

#[test]
fn test_skipifkey() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.reg_vx[1] = 3;
    let mut test_pc = test_emulator.pc;
    test_emulator.execute_opcode(OpCode::SkipIfKeyPressed { vx: 1 });
    assert!(test_emulator.pc == test_pc);
    test_emulator.keypad[3] = true;
    test_emulator.execute_opcode(OpCode::SkipIfKeyPressed { vx: 1 });
    assert!(test_emulator.pc == test_pc + 2);
    test_pc = test_emulator.pc;
    test_emulator.execute_opcode(OpCode::SkipIfKeyNotPressed { vx: 1 });
    assert!(test_emulator.pc == test_pc);
    test_emulator.keypad[3] = false;
    test_emulator.execute_opcode(OpCode::SkipIfKeyNotPressed { vx: 1 });
    assert!(test_emulator.pc == test_pc + 2);
}

#[test]
fn test_timers() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.reg_vx[1] = 3;
    test_emulator.execute_opcode(OpCode::SetDelayTimerToVx { vx: 1 });
    assert!(test_emulator.delay_timer == 3);
    test_emulator.execute_opcode(OpCode::SetVxToDelayTimer { vx: 2 });
    assert!(test_emulator.reg_vx[2] == 3);
    test_emulator.execute_opcode(OpCode::SetSoundTimerToVx { vx: 2 });
    assert!(test_emulator.sound_timer == 3);
}

#[test]
fn test_addtoindex() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.reg_vx[1] = 3;
    // Regular add
    test_emulator.reg_i = 8;
    test_emulator.execute_opcode(OpCode::AddToIndex { vx: 1 });
    assert!(test_emulator.reg_i == 11 && test_emulator.reg_vx[0xF] == 0);
    // Add with overflow
    test_emulator.reg_i = 0xFFF;
    test_emulator.execute_opcode(OpCode::AddToIndex { vx: 1 });
    assert!(test_emulator.reg_i == 2 && test_emulator.reg_vx[0xF] == 1);
}

#[test]
fn test_fontchar() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.reg_vx[1] = 3;
    test_emulator.execute_opcode(OpCode::FontCharacter { vx: 1 });
    assert!(test_emulator.reg_i == test_emulator.options.memory.font_start + (5 * 3));
}

#[test]
fn test_bincodeddecconv() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.reg_vx[1] = 0x9C;
    test_emulator.reg_i = 0x300;
    test_emulator.execute_opcode(OpCode::BinaryCodedDecimalConversion { vx: 1 });
    assert!(test_emulator.memory[0x300] == 1);
    assert!(test_emulator.memory[0x301] == 5);
    assert!(test_emulator.memory[0x302] == 6);
    assert!(test_emulator.memory[0x303] == 0);
}

#[test]
fn test_storememory() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.options.opcode.store_load_mem_use_i = false;
    test_emulator.reg_vx[0] = 11;
    test_emulator.reg_vx[1] = 13;
    test_emulator.reg_vx[2] = 15;
    test_emulator.reg_vx[3] = 17;
    test_emulator.reg_i = 0x300;
    test_emulator.execute_opcode(OpCode::StoreMemory { vx: 2 });
    assert!(test_emulator.memory[0x300] == 11);
    assert!(test_emulator.memory[0x301] == 13);
    assert!(test_emulator.memory[0x302] == 15);
    assert!(test_emulator.memory[0x303] == 0);
    assert!(test_emulator.reg_i == 0x300);

    test_emulator.options.opcode.store_load_mem_use_i = true;
    test_emulator.reg_i = 0x400;
    test_emulator.execute_opcode(OpCode::StoreMemory { vx: 2 });
    assert!(test_emulator.memory[0x400] == 11);
    assert!(test_emulator.memory[0x401] == 13);
    assert!(test_emulator.memory[0x402] == 15);
    assert!(test_emulator.memory[0x403] == 0);
    assert!(test_emulator.reg_i == 0x403);
}

#[test]
fn test_loadmemory() {
    let mut test_emulator = Emulator::new(&[0], &Chip8options::default());
    test_emulator.options.opcode.store_load_mem_use_i = false;
    test_emulator.reg_i = 0x300;
    test_emulator.memory[0x300] = 11;
    test_emulator.memory[0x301] = 13;
    test_emulator.memory[0x302] = 15;
    test_emulator.memory[0x303] = 17;
    test_emulator.execute_opcode(OpCode::LoadMemory { vx: 2 });
    assert!(test_emulator.reg_vx[0] == 11);
    assert!(test_emulator.reg_vx[1] == 13);
    assert!(test_emulator.reg_vx[2] == 15);
    assert!(test_emulator.reg_vx[3] == 0);
    assert!(test_emulator.reg_i == 0x300);

    test_emulator.options.opcode.store_load_mem_use_i = true;
    test_emulator.reg_vx[0] = 0;
    test_emulator.reg_vx[1] = 0;
    test_emulator.reg_vx[2] = 0;
    test_emulator.execute_opcode(OpCode::LoadMemory { vx: 2 });
    assert!(test_emulator.reg_vx[0] == 11);
    assert!(test_emulator.reg_vx[1] == 13);
    assert!(test_emulator.reg_vx[2] == 15);
    assert!(test_emulator.reg_vx[3] == 0);
    assert!(test_emulator.reg_i == 0x303);
}
