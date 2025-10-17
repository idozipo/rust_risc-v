use rust_risc_v::*;

/// Test fetching a LUI instruction from memory
#[test]
fn lui_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: LUI x1, 0x12345
    // opcode = 0110111, rd = 00001, imm[31:12] = 0x12345
    let lui_instruction: Word = (0x12345 << 12) | (1 << 7) | 0b0110111;
    mem.store_word(0x0, lui_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mem);
    assert_eq!(instruction, lui_instruction);
}

/// Basic LUI operation: load upper immediate
#[test]
fn lui_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LUI x2, 0xABCDE
    let lui_instruction: Word = (0xABCDE << 12) | (2 << 7) | 0b0110111;
    mem.store_word(0x0, lui_instruction);

    cpu.clock_cycle(&mem);

    // x2 = 0xABCDE000
    assert_eq!(cpu.reg[2], 0xABCDE000);
}

/// LUI should overwrite previous register value
#[test]
fn lui_overwrites_previous_value() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.reg[3] = 0xFFFFFFFF; // set some value first

    // LUI x3, 0x1
    let lui_instruction: Word = (0x1 << 12) | (3 << 7) | 0b0110111;
    mem.store_word(0x0, lui_instruction);

    cpu.clock_cycle(&mem);

    // Overwritten with upper immediate only
    assert_eq!(cpu.reg[3], 0x00001000);
}

/// LUI with zero immediate (should clear upper bits)
#[test]
fn lui_with_zero_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LUI x4, 0x0
    let lui_instruction: Word = (0x0 << 12) | (4 << 7) | 0b0110111;
    mem.store_word(0x0, lui_instruction);

    cpu.reg[4] = 0xDEADBEEF; // prefill

    cpu.clock_cycle(&mem);

    // Should zero out the register
    assert_eq!(cpu.reg[4], 0x00000000);
}

/// LUI with maximum positive immediate
#[test]
fn lui_with_max_positive_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LUI x5, 0x7FFFF
    let lui_instruction: Word = (0x7FFFF << 12) | (5 << 7) | 0b0110111;
    mem.store_word(0x0, lui_instruction);

    cpu.clock_cycle(&mem);

    // 0x7FFFF000
    assert_eq!(cpu.reg[5], 0x7FFFF000);
}

/// LUI with negative immediate (sign-extended)
#[test]
fn lui_with_negative_immediate_sign_extend() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LUI x6, 0xFFFFF (i.e., -1 << 12)
    let lui_instruction: Word = (0xFFFFF << 12) | (6 << 7) | 0b0110111;
    mem.store_word(0x0, lui_instruction);

    cpu.clock_cycle(&mem);

    // Should sign extend: 0xFFFFF000 treated as -4096
    assert_eq!(cpu.reg[6], 0xFFFFF000);
}

/// Writing to x0 should have no effect
#[test]
fn lui_write_to_x0_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LUI x0, 0x12345
    let lui_instruction: Word = (0x12345 << 12) | (0 << 7) | 0b0110111;
    mem.store_word(0x0, lui_instruction);

    cpu.clock_cycle(&mem);

    // Register x0 must remain 0
    assert_eq!(cpu.reg[0], 0);
}

/// LUI followed by an ADDI can create 32-bit constants (link test)
#[test]
fn lui_and_addi_link_behavior() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LUI x1, 0x12345
    let lui_instruction: Word = (0x12345 << 12) | (1 << 7) | 0b0110111;
    mem.store_word(0x0, lui_instruction);

    // ADDI x1, x1, 0x678 (add lower 12 bits)
    let addi_instruction: Word = (0x678 << 20) | (1 << 15) | (0b000 << 12) | (1 << 7) | 0b0010011;
    mem.store_word(0x4, addi_instruction);

    cpu.clock_cycle(&mem); // clock_cycle LUI
    cpu.clock_cycle(&mem); // clock_cycle ADDI

    // Result should combine upper and lower bits: 0x12345678
    assert_eq!(cpu.reg[1], 0x12345678);
}

/// Test fetching an AUIPC instruction from memory
#[test]
fn auipc_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: AUIPC x1, 0x12345
    // opcode = 0010111, rd = 00001, imm[31:12] = 0x12345
    let auipc_instruction: Word = (0x12345 << 12) | (1 << 7) | 0b0010111;
    mem.store_word(0x0, auipc_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mem);
    assert_eq!(instruction, auipc_instruction);
}

/// Basic AUIPC operation: PC + (imm << 12)
#[test]
fn auipc_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.pc = 0x1000;

    // AUIPC x2, 0xABCDE
    let auipc_instruction: Word = (0xABCDE << 12) | (2 << 7) | 0b0010111;
    mem.store_word(0x1000, auipc_instruction);

    cpu.clock_cycle(&mem);

    // Result: PC + (imm << 12)
    let expected = 0x1000u32.wrapping_add(0xABCDE000);
    assert_eq!(cpu.reg[2], expected);
}

/// AUIPC should add current PC, not next PC
#[test]
fn auipc_uses_current_pc() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // AUIPC x3, 0x1
    let auipc_instruction: Word = (0x1 << 12) | (3 << 7) | 0b0010111;
    mem.store_word(0x2000, auipc_instruction);

    cpu.pc = 0x2000;

    cpu.clock_cycle(&mem);

    // Result = 0x2000 + 0x1000 = 0x3000
    assert_eq!(cpu.reg[3], 0x3000);
}

/// AUIPC with zero immediate (should just copy PC)
#[test]
fn auipc_with_zero_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.pc = 0x4000;

    // AUIPC x4, 0x0
    let auipc_instruction: Word = (0x0 << 12) | (4 << 7) | 0b0010111;
    mem.store_word(0x4000, auipc_instruction);

    cpu.clock_cycle(&mem);

    // Result = PC + 0 = 0x4000
    assert_eq!(cpu.reg[4], 0x4000);
}

/// AUIPC with negative immediate (sign-extended)
#[test]
fn auipc_with_negative_immediate_sign_extend() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.pc = 0x1000;

    // AUIPC x5, 0xFFFFF (i.e. -1 << 12)
    let auipc_instruction: Word = (0xFFFFF << 12) | (5 << 7) | 0b0010111;
    mem.store_word(0x1000, auipc_instruction);

    cpu.clock_cycle(&mem);

    // Should sign extend: (imm << 12) = 0xFFFFF000 (-4096)
    let expected = 0x1000u32.wrapping_add(0xFFFFF000);
    assert_eq!(cpu.reg[5], expected);
}

/// AUIPC should handle large immediates correctly
#[test]
fn auipc_with_large_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.pc = 0x1000;

    // AUIPC x6, 0x7FFFF
    let auipc_instruction: Word = (0x7FFFF << 12) | (6 << 7) | 0b0010111;
    mem.store_word(0x1000, auipc_instruction);

    cpu.clock_cycle(&mem);

    let expected = 0x1000u32.wrapping_add(0x7FFFF000);
    assert_eq!(cpu.reg[6], expected);
}

/// Writing to x0 should have no effect
#[test]
fn auipc_write_to_x0_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.pc = 0x1234;

    // AUIPC x0, 0xABCDE
    let auipc_instruction: Word = (0xABCDE << 12) | (0 << 7) | 0b0010111;
    mem.store_word(0x1234, auipc_instruction);

    cpu.clock_cycle(&mem);

    // Register x0 must remain 0
    assert_eq!(cpu.reg[0], 0);
}
/// Writing to x0 should have no effect
#[test]
#[should_panic]
fn auipc_memory_out_of_bounds() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.pc = 0x80000000;

    // AUIPC x0, 0xABCDE
    let auipc_instruction: Word = (0xABCDE << 12) | (0 << 7) | 0b0010111;
    mem.store_word(0x80000000, auipc_instruction);

    cpu.clock_cycle(&mem);
}

/// AUIPC result should depend on the PC at execution time, not fetch time
#[test]
fn auipc_with_pc_change_before_execute() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // AUIPC x7, 0x123
    let auipc_instruction: Word = (0x123 << 12) | (7 << 7) | 0b0010111;
    mem.store_word(0x0, auipc_instruction);

    cpu.fetch_instruction(&mem);
    cpu.pc = 0x800; // changed before execute
    cpu.execute();
    cpu.increment_pc();

    // Result = 0x800 + (0x123 << 12) = 0x123800
    assert_eq!(cpu.reg[7], 0x123800);
}

// TODO:
// AUIPC combined with JALR can build PC-relative addresses (link test)
// #[test]
// fn auipc_and_jalr_link_behavior() {
//     let mut cpu: RISCV = RISCV::reset();
//     let mut mem: Memory = Memory::new();

//     // AUIPC x1, 0x100
//     let auipc_instruction: Word = (0x100 << 12) | (1 << 7) | 0b0010111;
//     mem.store_word(0x0, auipc_instruction);

//     // JALR x2, 0(x1) -> jumps to PC = x1 + 0
//     let jalr_instruction: Word = (0x0 << 20) | (1 << 15) | (0b000 << 12) | (2 << 7) | 0b1100111;
//     mem.store_word(0x4, jalr_instruction);

//     cpu.pc = 0x1000;

//     // Execute AUIPC first
//     cpu.clock_cycle(&mem);

//     // Next instruction
//     cpu.pc = 0x4;
//     cpu.clock_cycle(&mem);

//     // x1 = 0x1000 + (0x100 << 12) = 0x101000
//     // JALR jumps to x1 + 0
//     assert_eq!(cpu.reg[1], 0x101000);
//     assert_eq!(cpu.pc, 0x101000);
// }
