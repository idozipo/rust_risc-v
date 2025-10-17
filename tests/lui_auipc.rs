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

    let instruction: u32 = cpu.fetch_instruction_word(&mem);
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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem); // execute LUI
    cpu.execute(&mem); // execute ADDI

    // Result should combine upper and lower bits: 0x12345678
    assert_eq!(cpu.reg[1], 0x12345678);
}
