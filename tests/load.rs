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

    let instruction = cpu.fetch_instruction(&mem);
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

    cpu.clock_cycle(&mem);

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

    cpu.clock_cycle(&mem);

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

    cpu.clock_cycle(&mem);

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

    cpu.clock_cycle(&mem);

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

    cpu.clock_cycle(&mem);
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

    cpu.clock_cycle(&mem);

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

    cpu.clock_cycle(&mem);

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

    cpu.clock_cycle(&mem); // LW
    cpu.clock_cycle(&mem); // ADDI

    assert_eq!(cpu.reg[10], 0x10);
    assert_eq!(cpu.reg[11], 0x15); // 0x10 + 5
    assert_eq!(cpu.pc, 0x08);
}
