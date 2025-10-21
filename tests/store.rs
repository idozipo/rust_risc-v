use rust_risc_v::*;

/* -------------------- SW tests -------------------- */

/// SW instruction fetch
#[test]
fn sw_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SW x5, 0(x1)
    // funct3 = 010, opcode = 0100011
    // imm[11:5]=0000000, rs2=00101, rs1=00001, funct3=010, imm[4:0]=00000
    let sw_instr: Word = 0b0000000_00101_00001_010_00000_0100011;
    mem.store_word(0x0, sw_instr);

    let instruction = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, sw_instr);
}

/// SW basic store to memory
#[test]
fn sw_basic_store() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SW x6, 0(x2)
    let sw_instr: Word = 0b0000000_00110_00010_010_00000_0100011;
    mem.store_word(0x0, sw_instr);

    cpu.reg[2] = 0x100;
    cpu.reg[6] = 0xDEADBEEF;

    cpu.clock_cycle(&mut mem);

    assert_eq!(mem.fetch_word(0x100), 0xDEADBEEF);
    assert_eq!(cpu.pc, 4);
}

/// SW with positive offset
#[test]
fn sw_with_positive_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SW x7, 8(x3)
    // imm = 8 → imm[11:5]=0000000, imm[4:0]=01000
    let sw_instr: Word = 0b0000000_00111_00011_010_01000_0100011;
    mem.store_word(0x0, sw_instr);

    cpu.reg[3] = 0x200;
    cpu.reg[7] = 0xCAFEBABE;

    cpu.clock_cycle(&mut mem);

    assert_eq!(mem.fetch_word(0x208), 0xCAFEBABE);
    assert_eq!(cpu.pc, 4);
}

/// SW with negative offset
#[test]
fn sw_with_negative_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SW x8, -4(x4)
    // imm = -4 (0xFFC) → imm[11:5]=1111111, imm[4:0]=11100
    let sw_instr: Word = 0b1111111_01000_00100_010_11100_0100011;
    mem.store_word(0x0, sw_instr);

    cpu.reg[4] = 0x300;
    cpu.reg[8] = 0x12345678;

    cpu.clock_cycle(&mut mem);

    assert_eq!(mem.fetch_word(0x2FC), 0x12345678);
    assert_eq!(cpu.pc, 4);
}

/// SW overwrites previous memory value
#[test]
fn sw_overwrite_existing_value() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    let addr = 0x400;
    mem.store_word(addr, 0xAAAAAAAA); // initial value

    // SW x9, 0(x5)
    let sw_instr: Word = 0b0000000_01001_00101_010_00000_0100011;
    mem.store_word(0x0, sw_instr);

    cpu.reg[5] = addr as u32;
    cpu.reg[9] = 0xBBBBCCCC;

    cpu.clock_cycle(&mut mem);

    assert_eq!(mem.fetch_word(addr), 0xBBBBCCCC);
    assert_eq!(cpu.pc, 4);
}

/// SW with x0 as source → should write 0
#[test]
fn sw_from_x0_writes_zero() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SW x0, 0(x6)
    let sw_instr: Word = 0b0000000_00000_00110_010_00000_0100011;
    mem.store_word(0x0, sw_instr);

    cpu.reg[6] = 0x500;
    mem.store_word(0x500, 0xFFFFFFFF); // prefill memory

    cpu.clock_cycle(&mut mem);

    assert_eq!(mem.fetch_word(0x500), 0x00000000);
    assert_eq!(cpu.pc, 4);
}

/// SW to unaligned address → should panic
#[test]
#[should_panic]
fn sw_unaligned_address_panics() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SW x10, 2(x7)  (unaligned address)
    let sw_instr: Word = 0b0000000_01010_00111_010_00010_0100011;
    mem.store_word(0x0, sw_instr);

    cpu.reg[7] = 0x600;
    cpu.reg[10] = 0xABABABAB;

    cpu.clock_cycle(&mut mem);
}

/// SW multiple stores: ensure subsequent writes do not interfere
#[test]
fn sw_multiple_stores_independent() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SW x11, 0(x8)
    let sw_instr_1: Word = 0b0000000_01011_01000_010_00000_0100011;
    // SW x12, 4(x8)
    let sw_instr_2: Word = 0b0000000_01100_01000_010_00100_0100011;

    mem.store_word(0x0, sw_instr_1);
    mem.store_word(0x4, sw_instr_2);

    cpu.reg[8] = 0x700;
    cpu.reg[11] = 0x11112222;
    cpu.reg[12] = 0x33334444;

    cpu.clock_cycle(&mut mem); // first SW
    cpu.clock_cycle(&mut mem); // second SW

    assert_eq!(mem.fetch_word(0x700), 0x11112222);
    assert_eq!(mem.fetch_word(0x704), 0x33334444);
    assert_eq!(cpu.pc, 8);
}

/* -------------------- SH tests -------------------- */

/// SH instruction fetch
#[test]
fn sh_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SH x5, 0(x1)
    // funct3 = 001, opcode = 0100011
    // imm[11:5]=0000000, rs2=00101, rs1=00001, funct3=001, imm[4:0]=00000
    let sh_instr: Word = 0b0000000_00101_00001_001_00000_0100011;
    mem.store_word(0x0, sh_instr);

    let instruction = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, sh_instr);
}

/// SH basic store to memory
#[test]
fn sh_basic_store() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SH x6, 0(x2)
    let sh_instr: Word = 0b0000000_00110_00010_001_00000_0100011;
    mem.store_word(0x0, sh_instr);

    cpu.reg[2] = 0x100;
    cpu.reg[6] = 0xABCD1234;

    cpu.clock_cycle(&mut mem);

    // Only the lower 16 bits (0x1234) should be stored
    assert_eq!(mem.fetch_halfword(0x100), 0x1234);
    assert_eq!(cpu.pc, 4);
}

/// SH with positive offset
#[test]
fn sh_with_positive_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SH x7, 6(x3)
    // imm = 6 (000000000110)
    let sh_instr: Word = 0b0000000_00111_00011_001_00110_0100011;
    mem.store_word(0x0, sh_instr);

    cpu.reg[3] = 0x200;
    cpu.reg[7] = 0xCAFEBABE;

    cpu.clock_cycle(&mut mem);

    // Store 0xBABE to address 0x206
    assert_eq!(mem.fetch_halfword(0x206), 0xBABE);
    assert_eq!(cpu.pc, 4);
}

/// SH with negative offset
#[test]
fn sh_with_negative_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SH x8, -2(x4)
    // imm = -2 (0xFFE) → imm[11:5]=1111111, imm[4:0]=11110
    let sh_instr: Word = 0b1111111_01000_00100_001_11110_0100011;
    mem.store_word(0x0, sh_instr);

    cpu.reg[4] = 0x300;
    cpu.reg[8] = 0x12345678;

    cpu.clock_cycle(&mut mem);

    // Store halfword 0x5678 at address 0x2FE
    assert_eq!(mem.fetch_halfword(0x2FE), 0x5678);
    assert_eq!(cpu.pc, 4);
}

/// SH overwrites previous halfword
#[test]
fn sh_overwrite_existing_value() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    let addr = 0x400;
    mem.store_halfword(addr, 0xAAAA);

    // SH x9, 0(x5)
    let sh_instr: Word = 0b0000000_01001_00101_001_00000_0100011;
    mem.store_word(0x0, sh_instr);

    cpu.reg[5] = addr as u32;
    cpu.reg[9] = 0xBBBBCCCC;

    cpu.clock_cycle(&mut mem);

    assert_eq!(mem.fetch_halfword(addr), 0xCCCC);
    assert_eq!(cpu.pc, 4);
}

/// SH from x0 stores 0
#[test]
fn sh_from_x0_writes_zero() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SH x0, 0(x6)
    let sh_instr: Word = 0b0000000_00000_00110_001_00000_0100011;
    mem.store_word(0x0, sh_instr);

    cpu.reg[6] = 0x500;
    mem.store_halfword(0x500, 0xFFFF);

    cpu.clock_cycle(&mut mem);

    assert_eq!(mem.fetch_halfword(0x500), 0x0000);
    assert_eq!(cpu.pc, 4);
}

/// SH to unaligned address → should panic
#[test]
#[should_panic]
fn sh_unaligned_address_panics() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SH x10, 1(x7)  (unaligned address)
    let sh_instr: Word = 0b0000000_01010_00111_001_00001_0100011;
    mem.store_word(0x0, sh_instr);

    cpu.reg[7] = 0x600;
    cpu.reg[10] = 0xAABBCCDD;

    cpu.clock_cycle(&mut mem);
}

/// SH multiple stores independent
#[test]
fn sh_multiple_stores_independent() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SH x11, 0(x8)
    let sh_instr_1: Word = 0b0000000_01011_01000_001_00000_0100011;
    // SH x12, 2(x8)
    let sh_instr_2: Word = 0b0000000_01100_01000_001_00010_0100011;

    mem.store_word(0x0, sh_instr_1);
    mem.store_word(0x4, sh_instr_2);

    cpu.reg[8] = 0x700;
    cpu.reg[11] = 0x11112222;
    cpu.reg[12] = 0x33334444;

    cpu.clock_cycle(&mut mem); // SH x11, 0(x8)
    cpu.clock_cycle(&mut mem); // SH x12, 2(x8)

    assert_eq!(mem.fetch_halfword(0x700), 0x2222);
    assert_eq!(mem.fetch_halfword(0x702), 0x4444);
    assert_eq!(cpu.pc, 8);
}

/* -------------------- SB tests -------------------- */

/// SB instruction fetch
#[test]
fn sb_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SB x5, 0(x1)
    // funct3 = 000, opcode = 0100011
    // imm[11:5]=0000000, rs2=00101, rs1=00001, funct3=000, imm[4:0]=00000
    let sb_instr: Word = 0b0000000_00101_00001_000_00000_0100011;
    mem.store_word(0x0, sb_instr);

    let instruction = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, sb_instr);
}

/// SB basic store to memory
#[test]
fn sb_basic_store() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SB x6, 0(x2)
    let sb_instr: Word = 0b0000000_00110_00010_000_00000_0100011;
    mem.store_word(0x0, sb_instr);

    cpu.reg[2] = 0x100;
    cpu.reg[6] = 0xABCD1234;

    cpu.clock_cycle(&mut mem);

    // Only the lowest byte (0x34) should be stored
    assert_eq!(mem[0x100], 0x34);
    assert_eq!(cpu.pc, 4);
}

/// SB with positive offset
#[test]
fn sb_with_positive_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SB x7, 5(x3)
    // imm = 5 (000000000101)
    let sb_instr: Word = 0b0000000_00111_00011_000_00101_0100011;
    mem.store_word(0x0, sb_instr);

    cpu.reg[3] = 0x200;
    cpu.reg[7] = 0xCAFEBABE;

    cpu.clock_cycle(&mut mem);

    // Store least-significant byte (0xBE) to address 0x205
    assert_eq!(mem[0x205], 0xBE);
    assert_eq!(cpu.pc, 4);
}

/// SB with negative offset
#[test]
fn sb_with_negative_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SB x8, -3(x4)
    // imm = -3 (0xFFD) → imm[11:5]=1111111, imm[4:0]=11101
    let sb_instr: Word = 0b1111111_01000_00100_000_11101_0100011;
    mem.store_word(0x0, sb_instr);

    cpu.reg[4] = 0x300;
    cpu.reg[8] = 0x12345678;

    cpu.clock_cycle(&mut mem);

    // Store least-significant byte (0x78) at address 0x2FD
    assert_eq!(mem[0x2FD], 0x78);
    assert_eq!(cpu.pc, 4);
}

/// SB overwrites existing byte
#[test]
fn sb_overwrite_existing_value() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    let addr = 0x400;
    mem[addr] = 0xAA;

    // SB x9, 0(x5)
    let sb_instr: Word = 0b0000000_01001_00101_000_00000_0100011;
    mem.store_word(0x0, sb_instr);

    cpu.reg[5] = addr as u32;
    cpu.reg[9] = 0xBBBBCCDD;

    cpu.clock_cycle(&mut mem);

    assert_eq!(mem[addr], 0xDD);
    assert_eq!(cpu.pc, 4);
}

/// SB from x0 stores zero
#[test]
fn sb_from_x0_writes_zero() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SB x0, 0(x6)
    let sb_instr: Word = 0b0000000_00000_00110_000_00000_0100011;
    mem.store_word(0x0, sb_instr);

    cpu.reg[6] = 0x500;
    mem[0x500] = 0xFF;

    cpu.clock_cycle(&mut mem);

    assert_eq!(mem[0x500], 0x00);
    assert_eq!(cpu.pc, 4);
}

/// SB unaligned addresses are valid (since bytes have no alignment)
#[test]
fn sb_unaligned_addresses_are_valid() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SB x10, 3(x7)
    let sb_instr: Word = 0b0000000_01010_00111_000_00011_0100011;
    mem.store_word(0x0, sb_instr);

    cpu.reg[7] = 0x600;
    cpu.reg[10] = 0xAABBCCDD;

    cpu.clock_cycle(&mut mem);

    assert_eq!(mem[0x603], 0xDD);
    assert_eq!(cpu.pc, 4);
}

/// SB multiple stores independent
#[test]
fn sb_multiple_stores_independent() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SB x11, 0(x8)
    let sb_instr_1: Word = 0b0000000_01011_01000_000_00000_0100011;
    // SB x12, 1(x8)
    let sb_instr_2: Word = 0b0000000_01100_01000_000_00001_0100011;

    mem.store_word(0x0, sb_instr_1);
    mem.store_word(0x4, sb_instr_2);

    cpu.reg[8] = 0x700;
    cpu.reg[11] = 0x11112222;
    cpu.reg[12] = 0x33334444;

    cpu.clock_cycle(&mut mem); // SB x11, 0(x8)
    cpu.clock_cycle(&mut mem); // SB x12, 1(x8)

    assert_eq!(mem[0x700], 0x22);
    assert_eq!(mem[0x701], 0x44);
    assert_eq!(cpu.pc, 8);
}
