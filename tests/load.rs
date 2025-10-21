use rust_risc_v::*;

/* -------------------- LW tests -------------------- */

/// LW instruction fetch
#[test]
fn lw_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LW x5, 0(x1)
    // funct3 = 010, opcode = 0000011
    let lw_instruction: Word = 0b000000000000_00001_010_00101_0000011;
    mem.store_word(0x0, lw_instruction);

    let instruction = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, lw_instruction);
}

/// LW basic load from memory
#[test]
fn lw_basic_load() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LW x5, 0(x1)
    let lw_instr: Word = 0b000000000000_00001_010_00101_0000011;
    mem.store_word(0x0, lw_instr);

    // Setup memory and base register
    cpu.reg[1] = 0x100; // base
    mem.store_word(0x100, 0xDEADBEEF);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[5], 0xDEADBEEF);
    assert_eq!(cpu.pc, 4);
}

/// LW with positive offset
#[test]
fn lw_with_positive_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LW x6, 8(x2)
    let lw_instr: Word = 0b000000001000_00010_010_00110_0000011;
    mem.store_word(0x0, lw_instr);

    cpu.reg[2] = 0x200;
    mem.store_word(0x208, 0x12345678);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[6], 0x12345678);
    assert_eq!(cpu.pc, 4);
}

/// LW with negative offset
#[test]
fn lw_with_negative_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LW x7, -4(x3)
    // immediate = 0xFFC (-4 in 12-bit two’s complement)
    let lw_instr: Word = 0b111111111100_00011_010_00111_0000011;
    mem.store_word(0x0, lw_instr);

    cpu.reg[3] = 0x300;
    mem.store_word(0x2FC, 0xCAFEBABE);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[7], 0xCAFEBABE);
    assert_eq!(cpu.pc, 4);
}

/// LW sign-extension check (should not sign-extend, 32-bit load)
#[test]
fn lw_sign_extension_check() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LW x8, 0(x4)
    let lw_instr: Word = 0b000000000000_00100_010_01000_0000011;
    mem.store_word(0x0, lw_instr);

    cpu.reg[4] = 0x400;
    mem.store_word(0x400, 0x8000_0001);

    cpu.clock_cycle(&mut mem);

    // LW loads full 32-bit word as-is, no sign-extension change
    assert_eq!(cpu.reg[8], 0x8000_0001);
}

/// LW from unaligned address should panic
#[test]
#[should_panic]
fn lw_unaligned_address_panics() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LW x9, 2(x5) → unaligned address
    let lw_instr: Word = 0b000000000010_00101_010_01001_0000011;
    mem.store_word(0x0, lw_instr);

    cpu.reg[5] = 0x100; // base
    mem.store_word(0x102, 0x12345678);

    cpu.clock_cycle(&mut mem);
}

/// LW overwriting same register as base (x1 used as base and destination)
#[test]
fn lw_overwrite_base_register() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LW x1, 0(x1)
    let lw_instr: Word = 0b000000000000_00001_010_00001_0000011;
    mem.store_word(0x0, lw_instr);

    cpu.reg[1] = 0x500;
    mem.store_word(0x500, 0xFACE_FEED);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 0xFACE_FEED);
    assert_eq!(cpu.pc, 4);
}

/// LW reading zero register (x0) → should not write
#[test]
fn lw_to_x0_does_not_write() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LW x0, 0(x1)
    let lw_instr: Word = 0b000000000000_00001_010_00000_0000011;
    mem.store_word(0x0, lw_instr);

    cpu.reg[1] = 0x600;
    mem.store_word(0x600, 0xABCD1234);

    cpu.clock_cycle(&mut mem);

    // x0 always stays zero
    assert_eq!(cpu.reg[0], 0);
    assert_eq!(cpu.pc, 4);
}

/// LW followed by dependent instruction (pipeline-like check)
#[test]
fn lw_then_addi_dependency() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // 0x00: LW x10, 0(x1)
    // 0x04: ADDI x11, x10, 5
    let lw_x10_0x1: Word = 0b000000000000_00001_010_01010_0000011;
    let addi_x11_x10_5: Word = 0b000000000101_01010_000_01011_0010011;

    mem.store_word(0x00, lw_x10_0x1);
    mem.store_word(0x04, addi_x11_x10_5);

    cpu.reg[1] = 0x700;
    mem.store_word(0x700, 0x10);

    cpu.clock_cycle(&mut mem); // LW
    cpu.clock_cycle(&mut mem); // ADDI

    assert_eq!(cpu.reg[10], 0x10);
    assert_eq!(cpu.reg[11], 0x15); // 0x10 + 5
    assert_eq!(cpu.pc, 0x08);
}

/* -------------------- LH tests -------------------- */

/// LH instruction fetch
#[test]
fn lh_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LH x5, 0(x1)
    // funct3 = 001, opcode = 0000011
    let lh_instr: Word = 0b000000000000_00001_001_00101_0000011;
    mem.store_word(0x0, lh_instr);

    let instruction = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, lh_instr);
}

/// LH basic positive load
#[test]
fn lh_basic_load_positive() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LH x6, 0(x2)
    let lh_instr: Word = 0b000000000000_00010_001_00110_0000011;
    mem.store_word(0x0, lh_instr);

    cpu.reg[2] = 0x100;
    // Memory at 0x100 = 0x1234 (little-endian)
    mem.store_halfword(0x100, 0x1234);

    cpu.clock_cycle(&mut mem);

    // 0x1234 stays as 0x00001234 after sign extension
    assert_eq!(cpu.reg[6], 0x0000_1234);
    assert_eq!(cpu.pc, 4);
}

/// LH with negative number (sign-extended)
#[test]
fn lh_sign_extension_negative() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LH x7, 0(x3)
    let lh_instr: Word = 0b000000000000_00011_001_00111_0000011;
    mem.store_word(0x0, lh_instr);

    cpu.reg[3] = 0x200;
    mem.store_halfword(0x200, 0xF234); // negative 16-bit value

    cpu.clock_cycle(&mut mem);

    // Sign-extended: 0xFFFF_F234
    assert_eq!(cpu.reg[7], 0xFFFF_F234);
    assert_eq!(cpu.pc, 4);
}

/// LH with positive offset
#[test]
fn lh_with_positive_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LH x8, 4(x4)
    let lh_instr: Word = 0b000000000100_00100_001_01000_0000011;
    mem.store_word(0x0, lh_instr);

    cpu.reg[4] = 0x300;
    mem.store_halfword(0x304, 0x7FFF);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[8], 0x0000_7FFF);
    assert_eq!(cpu.pc, 4);
}

/// LH with negative offset
#[test]
fn lh_with_negative_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LH x9, -2(x5)
    let lh_instr: Word = 0b111111111110_00101_001_01001_0000011;
    mem.store_word(0x0, lh_instr);

    cpu.reg[5] = 0x400;
    mem.store_halfword(0x3FE, 0xBEEF);

    cpu.clock_cycle(&mut mem);

    // 0xBEEF sign-extends → 0xFFFF_BEEF
    assert_eq!(cpu.reg[9], 0xFFFF_BEEF);
    assert_eq!(cpu.pc, 4);
}

/// LH unaligned address → should panic
#[test]
#[should_panic]
fn lh_unaligned_address_panics() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LH x10, 1(x6)
    let lh_instr: Word = 0b000000000001_00110_001_01010_0000011;
    mem.store_word(0x0, lh_instr);

    cpu.reg[6] = 0x100;
    mem.store_halfword(0x101, 0xABCD);

    cpu.clock_cycle(&mut mem);
}

/// LH destination is x0 → should not write
#[test]
fn lh_to_x0_does_not_write() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LH x0, 0(x7)
    let lh_instr: Word = 0b000000000000_00111_001_00000_0000011;
    mem.store_word(0x0, lh_instr);

    cpu.reg[7] = 0x500;
    mem.store_halfword(0x500, 0xAAAA);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[0], 0);
    assert_eq!(cpu.pc, 4);
}

/* -------------------- LHU tests -------------------- */

/// LHU instruction fetch
#[test]
fn lhu_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LHU x5, 0(x1)
    // funct3 = 101, opcode = 0000011
    let lhu_instr: Word = 0b000000000000_00001_101_00101_0000011;
    mem.store_word(0x0, lhu_instr);

    let instruction = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, lhu_instr);
}

/// LHU basic positive load
#[test]
fn lhu_basic_load_positive() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LHU x6, 0(x2)
    let lhu_instr: Word = 0b000000000000_00010_101_00110_0000011;
    mem.store_word(0x0, lhu_instr);

    cpu.reg[2] = 0x600;
    mem.store_halfword(0x600, 0x1234);

    cpu.clock_cycle(&mut mem);

    // Zero-extended
    assert_eq!(cpu.reg[6], 0x0000_1234);
    assert_eq!(cpu.pc, 4);
}

/// LHU should zero-extend negative halfword
#[test]
fn lhu_zero_extension_from_negative() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LHU x7, 0(x3)
    let lhu_instr: Word = 0b000000000000_00011_101_00111_0000011;
    mem.store_word(0x0, lhu_instr);

    cpu.reg[3] = 0x700;
    mem.store_halfword(0x700, 0xF234); // 16-bit negative

    cpu.clock_cycle(&mut mem);

    // Zero-extended → 0x0000_F234
    assert_eq!(cpu.reg[7], 0x0000_F234);
    assert_eq!(cpu.pc, 4);
}

/// LHU with positive offset
#[test]
fn lhu_with_positive_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LHU x8, 6(x4)
    let lhu_instr: Word = 0b000000000110_00100_101_01000_0000011;
    mem.store_word(0x0, lhu_instr);

    cpu.reg[4] = 0x800;
    mem.store_halfword(0x806, 0xBEEF);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[8], 0x0000_BEEF);
    assert_eq!(cpu.pc, 4);
}

/// LHU unaligned address → should panic
#[test]
#[should_panic]
fn lhu_unaligned_address_panics() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LHU x9, 3(x5)
    let lhu_instr: Word = 0b000000000011_00101_101_01001_0000011;
    mem.store_word(0x0, lhu_instr);

    cpu.reg[5] = 0x900;
    mem.store_halfword(0x903, 0xABCD);

    cpu.clock_cycle(&mut mem);
}

/// LHU destination is x0 → should not write
#[test]
fn lhu_to_x0_does_not_write() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LHU x0, 0(x6)
    let lhu_instr: Word = 0b000000000000_00110_101_00000_0000011;
    mem.store_word(0x0, lhu_instr);

    cpu.reg[6] = 0xA00;
    mem.store_halfword(0xA00, 0x1111);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[0], 0);
    assert_eq!(cpu.pc, 4);
}

/* -------------------- LB tests -------------------- */

/// LB instruction fetch
#[test]
fn lb_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LB x5, 0(x1)
    // funct3 = 000, opcode = 0000011
    let lb_instr: Word = 0b000000000000_00001_000_00101_0000011;
    mem.store_word(0x0, lb_instr);

    let instruction = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, lb_instr);
}

/// LB basic positive load
#[test]
fn lb_basic_load_positive() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LB x6, 0(x2)
    let lb_instr: Word = 0b000000000000_00010_000_00110_0000011;
    mem.store_word(0x0, lb_instr);

    cpu.reg[2] = 0x100;
    mem[0x100] = 0x12;

    cpu.clock_cycle(&mut mem);

    // 0x12 sign-extends to 0x0000_0012
    assert_eq!(cpu.reg[6], 0x0000_0012);
    assert_eq!(cpu.pc, 4);
}

/// LB loads a negative byte and sign-extends
#[test]
fn lb_sign_extension_negative() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LB x7, 0(x3)
    let lb_instr: Word = 0b000000000000_00011_000_00111_0000011;
    mem.store_word(0x0, lb_instr);

    cpu.reg[3] = 0x200;
    mem[0x200] = 0xF2; // negative 8-bit value

    cpu.clock_cycle(&mut mem);

    // Sign-extended: 0xFFFF_FFF2
    assert_eq!(cpu.reg[7], 0xFFFF_FFF2);
    assert_eq!(cpu.pc, 4);
}

/// LB with positive offset
#[test]
fn lb_with_positive_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LB x8, 3(x4)
    let lb_instr: Word = 0b000000000011_00100_000_01000_0000011;
    mem.store_word(0x0, lb_instr);

    cpu.reg[4] = 0x300;
    mem[0x303] = 0x7F;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[8], 0x0000_007F);
    assert_eq!(cpu.pc, 4);
}

/// LB with negative offset
#[test]
fn lb_with_negative_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LB x9, -1(x5)
    let lb_instr: Word = 0b111111111111_00101_000_01001_0000011;
    mem.store_word(0x0, lb_instr);

    cpu.reg[5] = 0x400;
    mem[0x3FF] = 0x80; // negative 8-bit value (-128)

    cpu.clock_cycle(&mut mem);

    // Sign-extended: 0xFFFF_FF80
    assert_eq!(cpu.reg[9], 0xFFFF_FF80);
    assert_eq!(cpu.pc, 4);
}

/// LB destination is x0 → should not write
#[test]
fn lb_to_x0_does_not_write() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LB x0, 0(x6)
    let lb_instr: Word = 0b000000000000_00110_000_00000_0000011;
    mem.store_word(0x0, lb_instr);

    cpu.reg[6] = 0x500;
    mem[0x500] = 0xAB;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[0], 0);
    assert_eq!(cpu.pc, 4);
}

/* -------------------- LBU tests -------------------- */

/// LBU instruction fetch
#[test]
fn lbu_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LBU x5, 0(x1)
    // funct3 = 100, opcode = 0000011
    let lbu_instr: Word = 0b000000000000_00001_100_00101_0000011;
    mem.store_word(0x0, lbu_instr);

    let instruction = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, lbu_instr);
}

/// LBU basic positive load
#[test]
fn lbu_basic_load_positive() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LBU x6, 0(x2)
    let lbu_instr: Word = 0b000000000000_00010_100_00110_0000011;
    mem.store_word(0x0, lbu_instr);

    cpu.reg[2] = 0x600;
    mem[0x600] = 0x12;

    cpu.clock_cycle(&mut mem);

    // Zero-extended
    assert_eq!(cpu.reg[6], 0x0000_0012);
    assert_eq!(cpu.pc, 4);
}

/// LBU loads a negative byte and zero-extends
#[test]
fn lbu_zero_extension_from_negative() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LBU x7, 0(x3)
    let lbu_instr: Word = 0b000000000000_00011_100_00111_0000011;
    mem.store_word(0x0, lbu_instr);

    cpu.reg[3] = 0x700;
    mem[0x700] = 0xF2; // 0xF2 = -14 signed

    cpu.clock_cycle(&mut mem);

    // Zero-extended → 0x0000_00F2
    assert_eq!(cpu.reg[7], 0x0000_00F2);
    assert_eq!(cpu.pc, 4);
}

/// LBU with positive offset
#[test]
fn lbu_with_positive_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LBU x8, 5(x4)
    let lbu_instr: Word = 0b000000000101_00100_100_01000_0000011;
    mem.store_word(0x0, lbu_instr);

    cpu.reg[4] = 0x800;
    mem[0x805] = 0xA5;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[8], 0x0000_00A5);
    assert_eq!(cpu.pc, 4);
}

/// LBU destination is x0 → should not write
#[test]
fn lbu_to_x0_does_not_write() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // LBU x0, 0(x6)
    let lbu_instr: Word = 0b000000000000_00110_100_00000_0000011;
    mem.store_word(0x0, lbu_instr);

    cpu.reg[6] = 0xA00;
    mem[0xA00] = 0x99;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[0], 0);
    assert_eq!(cpu.pc, 4);
}
