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

/// Test that a JALR instruction can be fetched correctly
#[test]
fn jalr_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: JALR x1, x2, 8
    // Encoding fields:
    // imm[11:0] | rs1 | funct3=000 | rd | opcode=1100111
    // imm=8 (0b000000001000)
    let jalr_instruction: Word = 0b000000001000_00010_000_00001_1100111;
    mem.store_word(0x0, jalr_instruction);

    let instruction = cpu.fetch_instruction(&mem);
    assert_eq!(instruction, jalr_instruction);
}

/// Basic JALR forward jump using register base
#[test]
fn jalr_basic_forward_jump() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Set base register x2 = 100
    cpu.reg[2] = 100;

    // JALR x1, x2, 12  → target = x2 + 12 = 112
    let jalr_instruction: Word = 0b000000001100_00010_000_00001_1100111;
    mem.store_word(0x0, jalr_instruction);

    cpu.clock_cycle(&mem);

    // After execution:
    // x1 = return address = PC + 4 = 4
    // PC = 112
    assert_eq!(cpu.reg[1], 4);
    assert_eq!(cpu.pc, 112);
}

/// JALR with negative offset
#[test]
fn jalr_negative_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.pc = 0x100;
    cpu.reg[2] = 0x120;

    // JALR x1, x2, -8 → target = 0x120 - 8 = 0x118
    let jalr_instruction: Word = 0b111111111000_00010_000_00001_1100111;
    mem.store_word(0x100, jalr_instruction);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 0x118);
    assert_eq!(cpu.reg[1], 0x104);
}

/// JALR writing to nonzero rd
#[test]
fn jalr_to_nonzero_rd() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.reg[5] = 0x80;

    // JALR x10, x5, 16 → jump to 0x90, link in x10
    let jalr_instruction: Word = 0b000000010000_00101_000_01010_1100111;
    mem.store_word(0x0, jalr_instruction);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 0x90);
    assert_eq!(cpu.reg[10], 4);
}

/// JALR with rd = x0 (no link)
#[test]
fn jalr_rd_x0_no_link_written() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.reg[1] = 0x200;

    // JALR x0, x1, 8 → jump but don’t link
    let jalr_instruction: Word = 0b000000001000_00001_000_00000_1100111;
    mem.store_word(0x0, jalr_instruction);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 0x208);
    assert_eq!(cpu.reg[0], 0); // still zero
}

/// JALR clears LSB of target address (alignment)
#[test]
fn jalr_alignment_mask() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.reg[2] = 101; // odd base address

    // JALR x1, x2, 8 → target = (101 + 8) & !1 = 108
    let jalr_instruction: Word = 0b000000001000_00010_000_00001_1100111;
    mem.store_word(0x0, jalr_instruction);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 108); // aligned down
    assert_eq!(cpu.reg[1], 4);
}

/// JALR + JAL combined jump chain test
#[test]
fn jalr_and_jal_combined() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Layout:
    // 0x00: JAL  x1, 12      → jump to 0x0C
    // 0x04: LUI  x2, 1      → skipped
    // 0x08: LUI  x3, 2      → skipped
    // 0x0C: LUI  x4, 3      → executed
    // 0x10: JALR x5, x4, 4  → x4=0x3000, target=0x3004

    let jal_x1 = 0b0_0000000110_0_00000000_00001_1101111; // +12
    let lui_x2 = 0b00000000000000000001_00010_0110111;
    let lui_x3 = 0b00000000000000000010_00011_0110111;
    let lui_x4 = 0b00000000000000000011_00100_0110111;
    let jalr_x5 = 0b000000000100_00100_000_00101_1100111;

    mem.store_word(0x00, jal_x1);
    mem.store_word(0x04, lui_x2);
    mem.store_word(0x08, lui_x3);
    mem.store_word(0x0C, lui_x4);
    mem.store_word(0x10, jalr_x5);

    cpu.clock_cycle(&mem); // JAL
    cpu.clock_cycle(&mem); // LUI x4
    cpu.clock_cycle(&mem); // JALR x5, x4, 4

    assert_eq!(cpu.reg[1], 0x04);
    assert_eq!(cpu.reg[4], 0x3000);
    assert_eq!(cpu.pc, 0x3004);
    assert_eq!(cpu.reg[5], 0x14);
}

/// JALR + LUI interaction
#[test]
fn jalr_with_lui_interaction() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // 0x00: LUI x2, 1       → x2 = 0x1000
    // 0x04: JALR x1, x2, 12 → jump to 0x100C
    // 0x08: LUI x3, 2       → skipped
    // 0x100C: LUI x4, 3

    let lui_x2 = 0b00000000000000000001_00010_0110111;
    let jalr_x1 = 0b000000001100_00010_000_00001_1100111;
    let lui_x3 = 0b00000000000000000010_00011_0110111;
    let lui_x4 = 0b00000000000000000011_00100_0110111;

    mem.store_word(0x00, lui_x2);
    mem.store_word(0x04, jalr_x1);
    mem.store_word(0x08, lui_x3);
    mem.store_word(0x100C, lui_x4);

    cpu.clock_cycle(&mem); // LUI
    cpu.clock_cycle(&mem); // JALR
    cpu.clock_cycle(&mem); // LUI at 0x100C

    assert_eq!(cpu.reg[2], 0x1000);
    assert_eq!(cpu.reg[3], 0);
    assert_eq!(cpu.reg[4], 0x3000);
    assert_eq!(cpu.pc, 0x1010);
    assert_eq!(cpu.reg[1], 0x08);
}

/// JALR + AUIPC interaction
#[test]
fn jalr_with_auipc_interaction() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // 0x00: AUIPC x5, 1   → x5 = PC + (1 << 12) = 0x1000
    // 0x04: JALR x6, x5, 8 → jump to 0x1008, x6 = return address (0x08)
    // 0x08: LUI x7, 2     → skipped
    // 0x1008: LUI x8, 3

    let auipc_x5 = 0b00000000000000000001_00101_0010111;
    let jalr_x6 = 0b000000001000_00101_000_00110_1100111;
    let lui_x7 = 0b00000000000000000010_00111_0110111;
    let lui_x8 = 0b00000000000000000011_01000_0110111;

    mem.store_word(0x00, auipc_x5);
    mem.store_word(0x04, jalr_x6);
    mem.store_word(0x08, lui_x7);
    mem.store_word(0x1008, lui_x8);

    cpu.clock_cycle(&mem); // AUIPC
    cpu.clock_cycle(&mem); // JALR
    cpu.clock_cycle(&mem); // LUI x8

    assert_eq!(cpu.reg[5], 0x1000);
    assert_eq!(cpu.reg[6], 0x08);
    assert_eq!(cpu.reg[7], 0);
    assert_eq!(cpu.reg[8], 0x3000);
    assert_eq!(cpu.pc, 0x100C);
}

/// JALR with unaligned jump target (should panic)
#[test]
#[should_panic]
fn jalr_illegal_unaligned_target_panics() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // x2 = 3, imm = 1 → target = 4
    cpu.reg[2] = 3;

    let jalr_instruction: Word = 0b000000000000_00010_000_00001_1100111;
    mem.store_word(0x0, jalr_instruction);

    cpu.clock_cycle(&mem); // Emulator should panic due to unaligned PC
}

/// Full chain JAL + JALR return test
#[test]
fn jal_and_jalr_return_sequence() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Layout:
    // 0x00: JAL x1, 12      → jump to 0x0C
    // 0x04: (skipped)
    // 0x08: (skipped)
    // 0x0C: JALR x0, x1, 0 → return (PC = x1 + 0)

    let jal = 0b0_0000000110_0_00000000_00001_1101111;
    let jalr = 0b000000000000_00001_000_00000_1100111;

    mem.store_word(0x00, jal);
    mem.store_word(0x0C, jalr);

    cpu.clock_cycle(&mem); // JAL
    cpu.clock_cycle(&mem); // JALR (return)

    assert_eq!(cpu.pc, 0x04); // returned to link address
}

#[test]
fn jal_and_jalr_combined_with_regular_instructions() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Program Instructions:
    // 0x00: ADDI x10, x0, 5
    // 0x04: ADDI x11, x0, 3
    // 0x08: JAL x1, 0x10 (add_func)
    // 0x0C: ADDI x0, x0, 0 (end marker)
    // 0x10: ADD x10, x10, x11 (add_func)
    // 0x14: JALR x0, x1, 0

    let addi_a0_5: Word = 0b000000000101_00000_000_01010_0010011; // addi x10, x0, 5
    let addi_a1_3: Word = 0b000000000011_00000_000_01011_0010011; // addi x11, x0, 3
    let jal_to_add_func: Word = 0b0_0000000100_0_00000000_00001_1101111; // jal x1, 8
    let end_marker: Word = 0b000000000000_00000_000_00000_0010011; // addi x0, x0, 0
    let add_func: Word = 0b0000000_01011_01010_000_01010_0110011; // add x10, x10, x11
    let jalr_return: Word = 0b000000000000_00001_000_00000_1100111; // jalr x0, x1, 0

    mem.store_word(0x00, addi_a0_5);
    mem.store_word(0x04, addi_a1_3);
    mem.store_word(0x08, jal_to_add_func);
    mem.store_word(0x0C, end_marker);
    mem.store_word(0x10, add_func);
    mem.store_word(0x14, jalr_return);

    // Execute instructions
    cpu.clock_cycle(&mem); // ADDI x10, x0, 5
    cpu.clock_cycle(&mem); // ADDI x11, x0, 3
    cpu.clock_cycle(&mem); // JAL to add_func
    cpu.clock_cycle(&mem); // ADD x10, x10, x11
    cpu.clock_cycle(&mem); // JALR return

    // After execution, x10 should contain 8 (5 + 3)
    assert_eq!(cpu.reg[10], 8);
    assert_eq!(cpu.pc, 0x0C); // Returned to instruction after JAL
    assert!(cpu.reg[1] == 0x0C); // Link register should have return address
}
