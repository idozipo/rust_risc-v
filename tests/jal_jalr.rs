use rust_risc_v::*;

/// Test that a JAL instruction can be fetched correctly
#[test]
fn jal_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: JAL x1, offset = 8
    // Encoding fields (simplified):
    // imm[20|10:1|11|19:12], rd=00001, opcode=1101111
    // For a small positive offset like 8:
    // imm = 0b00000000000100000000 (represents 8)
    let jal_instruction: Word = 0b000000000001_00000000_00001_1101111;
    mem.store_word(0x0, jal_instruction);

    let instruction = cpu.fetch_instruction(&mem);
    assert_eq!(instruction, jal_instruction);
}

/// Basic JAL forward jump test
#[test]
fn jal_basic_forward_jump() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // JAL x1, +8  (PC + 8)
    // PC starts at 0, so we’ll jump to address 0x8
    let jal_instruction: Word = 0b0_0000000100_0_00000000_00001_1101111; // imm=8
    mem.store_word(0x0, jal_instruction);

    cpu.clock_cycle(&mem);

    // After execution:
    // x1 = return address = PC + 4 = 4
    // PC = 0 + 8 = 8
    assert_eq!(cpu.reg[1], 4);
    assert_eq!(cpu.pc, 8);
}

/// JAL backward jump (negative offset)
#[test]
fn jal_backward_jump() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Place JAL at address 0x10
    cpu.pc = 0x10;

    // JAL x1, -8 (offset = -8)
    // The jump target = 0x10 - 8 = 0x08
    // Encode offset properly with sign bit
    // For test purposes we can simulate an instruction that produces -8
    let jal_instruction: Word = 0b1_1111111100_1_11111111_00001_1101111; // imm = -8
    mem.store_word(0x10, jal_instruction);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 0x08);
    assert_eq!(cpu.reg[1], 0x14); // return address = 0x10 + 4
}

/// JAL jump and link with non-zero destination register
#[test]
fn jal_to_nonzero_rd() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // JAL x5, 12  → jump to PC + 12, store return address in x5
    let jal_instruction: Word = 0b0_0000000110_0_00000000_00101_1101111;
    mem.store_word(0x0, jal_instruction);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 12);
    assert_eq!(cpu.reg[5], 4);
}

/// JAL with rd = x0 (no link)
#[test]
fn jal_rd_x0_no_link_written() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // JAL x0, 16 → jump but do not save return address
    let jal_instruction: Word = 0b0_0000001000_0_00000000_00000_1101111;
    mem.store_word(0x0, jal_instruction);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 16);
    assert_eq!(cpu.reg[0], 0);
}

/// JAL long forward jump (larger offset)
#[test]
fn jal_large_forward_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // JAL x1, +1024 (0x400)
    let jal_instruction: Word = 0b0_1000000000_0_00000000_00001_1101111;
    mem.store_word(0x0, jal_instruction);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 0x400);
    assert_eq!(cpu.reg[1], 4);
}

/// JAL long backward jump (negative offset)
#[test]
fn jal_large_negative_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Start PC at 0x800
    cpu.pc = 0x800;

    // JAL x1, -512 (0xFFFF_FE00)
    let jal_instruction: Word = 0b1_1100000000_1_11111111_00001_1101111;
    mem.store_word(0x800, jal_instruction);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 0x600); // 0x800 - 0x200
    assert_eq!(cpu.reg[1], 0x804); // return address
}

/// JAL + LUI interaction (load address constant, then jump)
#[test]
fn jal_with_lui_interaction() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Address layout:
    // 0x00: LUI x2, 0x1 (x2 = 0x1000)
    // 0x04: JAL x1, 8 (jump to 0x0C)
    // 0x08: LUI x3, 0x2 (skipped if JAL works)
    // 0x0C: LUI x4, 0x3

    // LUI x2, 0x1
    let lui_x2 = 0b00000000000000000001_00010_0110111;
    mem.store_word(0x00, lui_x2);

    // JAL x1, 8 (PC + 8)
    let jal_x1 = 0b0_0000000100_0_00000000_00001_1101111;
    mem.store_word(0x04, jal_x1);

    // LUI x3, 0x2 (should be skipped)
    let lui_x3 = 0b00000000000000000010_00011_0110111;
    mem.store_word(0x08, lui_x3);

    // LUI x4, 0x3 (target)
    let lui_x4 = 0b00000000000000000011_00100_0110111;
    mem.store_word(0x0C, lui_x4);

    cpu.clock_cycle(&mem); // LUI x2
    cpu.clock_cycle(&mem); // JAL (to 0x0C)
    cpu.clock_cycle(&mem); // LUI x4

    assert_eq!(cpu.reg[2], 0x1000);
    assert_eq!(cpu.reg[3], 0); // skipped
    assert_eq!(cpu.reg[4], 0x3000);
    assert_eq!(cpu.pc, 0x10);
    assert_eq!(cpu.reg[1], 0x08); // link address
}

/// JAL with AUIPC interaction
#[test]
fn jal_with_auipc_interaction() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // 0x00: AUIPC x5, 1   → x5 = PC + (1 << 12) = 0x1000
    // 0x04: JAL x6, 8     → jump to 0x0C, x6 = return address (0x08)
    // 0x08: LUI x7, 2     → should be skipped
    // 0x0C: LUI x8, 3     → executed after jump

    let auipc_x5 = 0b00000000000000000001_00101_0010111;
    mem.store_word(0x00, auipc_x5);

    let jal_x6 = 0b0_0000000100_0_00000000_00110_1101111;
    mem.store_word(0x04, jal_x6);

    let lui_x7 = 0b000000000000000000010_00111_0110111;
    mem.store_word(0x08, lui_x7);

    let lui_x8 = 0b00000000000000000011_01000_0110111;
    mem.store_word(0x0C, lui_x8);

    cpu.clock_cycle(&mem); // AUIPC
    cpu.clock_cycle(&mem); // JAL
    cpu.clock_cycle(&mem); // LUI x8

    assert_eq!(cpu.reg[5], 0x1000);
    assert_eq!(cpu.reg[6], 0x08); // return address
    assert_eq!(cpu.reg[7], 0); // skipped
    assert_eq!(cpu.reg[8], 0x3000);
    assert_eq!(cpu.pc, 0x10);
}

/// JAL with an illegal offset not 4-byte aligned (should panic)
#[test]
#[should_panic]
fn jal_illegal_unaligned_jump_panics() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Force an illegal jump target that is not 4-byte aligned (e.g., 2)
    let jal_instruction: Word = 0b0_0000000001_0_00000000_00001_1101111;
    mem.store_word(0x0, jal_instruction);

    cpu.clock_cycle(&mem); // Emulator should panic or raise an exception
}
