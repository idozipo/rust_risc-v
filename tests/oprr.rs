use rust_risc_v::*;

/// Test that an ADD instruction can be fetched correctly
#[test]
fn add_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: ADD x1, x2, x3
    // funct7=0000000, rs2=00011, rs1=00010, funct3=000, rd=00001, opcode=0110011
    let add_instruction: Word = 0b0000000_00011_00010_000_00001_0110011;
    mem.store_word(0x0, add_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mem);
    assert_eq!(instruction, add_instruction);
}

/// Basic ADD operation test
#[test]
fn add_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ADD x1, x2, x3
    let add_instruction: Word = 0b0000000_00011_00010_000_00001_0110011;
    mem.store_word(0x0, add_instruction);

    cpu.reg[2] = 5; // x2
    cpu.reg[3] = 7; // x3

    cpu.clock_cycle(&mem);

    // 5 + 7 = 12
    assert_eq!(cpu.reg[1], 12);
}

/// ADD with negative numbers (signed)
#[test]
fn add_with_negative_operands() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ADD x4, x5, x6
    let add_instruction: Word = 0b0000000_00110_00101_000_00100_0110011;
    mem.store_word(0x0, add_instruction);

    // x5 = -10, x6 = 3
    cpu.reg[5] = (-10i32) as u32;
    cpu.reg[6] = 3;

    cpu.clock_cycle(&mem);

    // Expected result: -7
    assert_eq!(cpu.reg[4] as i32, -7);
}

/// ADD with both negative operands
#[test]
fn add_two_negative_numbers() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ADD x7, x8, x9
    let add_instruction: Word = 0b0000000_01001_01000_000_00111_0110011;
    mem.store_word(0x0, add_instruction);

    // x8 = -5, x9 = -7
    cpu.reg[8] = (-5i32) as u32;
    cpu.reg[9] = (-7i32) as u32;

    cpu.clock_cycle(&mem);

    // -5 + -7 = -12
    assert_eq!(cpu.reg[7] as i32, -12);
}

/// ADD overflow wraps around (as per RISC-V spec)
#[test]
fn add_overflow_wraps_around() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ADD x10, x11, x12
    let add_instruction: Word = 0b0000000_01100_01011_000_01010_0110011;
    mem.store_word(0x0, add_instruction);

    // x11 = u32::MAX, x12 = 1
    cpu.reg[11] = u32::MAX;
    cpu.reg[12] = 1;

    cpu.clock_cycle(&mem);

    // Overflow wraps around: 0xFFFF_FFFF + 1 = 0
    assert_eq!(cpu.reg[10], 0);
}

/// ADD where one operand is zero (x0)
#[test]
fn add_with_x0_operand() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ADD x1, x0, x2 → x1 = 0 + x2 = x2
    let add_instruction: Word = 0b0000000_00010_00000_000_00001_0110011;
    mem.store_word(0x0, add_instruction);

    cpu.reg[2] = 99;

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.reg[1], 99);
}

/// ADD with destination register x0 (should not modify x0)
#[test]
fn add_write_to_x0_is_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ADD x0, x1, x2
    let add_instruction: Word = 0b0000000_00010_00001_000_00000_0110011;
    mem.store_word(0x0, add_instruction);

    cpu.reg[1] = 42;
    cpu.reg[2] = 58;

    cpu.clock_cycle(&mem);

    // x0 is hardwired to zero
    assert_eq!(cpu.reg[0], 0);
}

/// ADD with large values (testing wrapping arithmetic)
#[test]
fn add_with_large_values() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ADD x3, x4, x5
    let add_instruction: Word = 0b0000000_00101_00100_000_00011_0110011;
    mem.store_word(0x0, add_instruction);

    cpu.reg[4] = 0xFFFF_0000;
    cpu.reg[5] = 0x0000_FFFF;

    cpu.clock_cycle(&mem);

    // 0xFFFF_0000 + 0x0000_FFFF = 0xFFFF_FFFF
    assert_eq!(cpu.reg[3], 0xFFFF_FFFF);
}

/// ADD between same registers (e.g., x1 = x1 + x1)
#[test]
fn add_with_same_source_registers() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ADD x1, x1, x1 (effectively x1 = x1 * 2)
    let add_instruction: Word = 0b0000000_00001_00001_000_00001_0110011;
    mem.store_word(0x0, add_instruction);

    cpu.reg[1] = 123;

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.reg[1], 246);
}

/// ADD when all registers are zero
#[test]
fn add_all_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ADD x31, x0, x0
    let add_instruction: Word = 0b0000000_00000_00000_000_11111_0110011;
    mem.store_word(0x0, add_instruction);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.reg[31], 0);
}

/// Test that a SUB instruction can be fetched correctly
#[test]
fn sub_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: SUB x1, x2, x3
    // funct7=0100000, rs2=00011, rs1=00010, funct3=000, rd=00001, opcode=0110011
    let sub_instruction: Word = 0b0100000_00011_00010_000_00001_0110011;
    mem.store_word(0x0, sub_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mem);
    assert_eq!(instruction, sub_instruction);
}

/// Basic SUB operation test
#[test]
fn sub_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SUB x1, x2, x3
    let sub_instruction: Word = 0b0100000_00011_00010_000_00001_0110011;
    mem.store_word(0x0, sub_instruction);

    cpu.reg[2] = 10; // x2
    cpu.reg[3] = 4; // x3

    cpu.clock_cycle(&mem);

    // 10 - 4 = 6
    assert_eq!(cpu.reg[1], 6);
}

/// SUB with negative operands
#[test]
fn sub_with_negative_operands() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SUB x4, x5, x6
    let sub_instruction: Word = 0b0100000_00110_00101_000_00100_0110011;
    mem.store_word(0x0, sub_instruction);

    // x5 = -10, x6 = 3
    cpu.reg[5] = (-10i32) as u32;
    cpu.reg[6] = 3;

    cpu.clock_cycle(&mem);

    // Expected: -10 - 3 = -13
    assert_eq!(cpu.reg[4] as i32, -13);
}

/// SUB with both negative operands
#[test]
fn sub_two_negative_numbers() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SUB x7, x8, x9
    let sub_instruction: Word = 0b0100000_01001_01000_000_00111_0110011;
    mem.store_word(0x0, sub_instruction);

    // x8 = -5, x9 = -7
    cpu.reg[8] = (-5i32) as u32;
    cpu.reg[9] = (-7i32) as u32;

    cpu.clock_cycle(&mem);

    // -5 - (-7) = 2
    assert_eq!(cpu.reg[7] as i32, 2);
}

/// SUB underflow wraps around (as per RISC-V spec)
#[test]
fn sub_underflow_wraps_around() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SUB x10, x11, x12
    let sub_instruction: Word = 0b0100000_01100_01011_000_01010_0110011;
    mem.store_word(0x0, sub_instruction);

    // x11 = 0, x12 = 1
    cpu.reg[11] = 0;
    cpu.reg[12] = 1;

    cpu.clock_cycle(&mem);

    // 0 - 1 wraps to 0xFFFF_FFFF
    assert_eq!(cpu.reg[10], 0xFFFF_FFFF);
}

/// SUB where one operand is zero (x0)
#[test]
fn sub_with_x0_operand() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SUB x1, x0, x2 → x1 = 0 - x2 = -x2
    let sub_instruction: Word = 0b0100000_00010_00000_000_00001_0110011;
    mem.store_word(0x0, sub_instruction);

    cpu.reg[2] = 99;

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.reg[1] as i32, -99);
}

/// SUB with destination register x0 (should not modify x0)
#[test]
fn sub_write_to_x0_is_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SUB x0, x1, x2
    let sub_instruction: Word = 0b0100000_00010_00001_000_00000_0110011;
    mem.store_word(0x0, sub_instruction);

    cpu.reg[1] = 42;
    cpu.reg[2] = 58;

    cpu.clock_cycle(&mem);

    // x0 must stay zero
    assert_eq!(cpu.reg[0], 0);
}

/// SUB with large unsigned values
#[test]
fn sub_with_large_values() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SUB x3, x4, x5
    // Correction: rs1 = x4 (00100), rs2 = x5 (00101)
    let sub_instruction: Word = 0b0100000_00101_00100_000_00011_0110011;
    mem.store_word(0x0, sub_instruction);

    cpu.reg[4] = 0x0000_FFFF;
    cpu.reg[5] = 0xFFFF_0000;

    cpu.clock_cycle(&mem);

    // 0x0000_FFFF - 0xFFFF_0000 = 0x0001_FFFF (wrap-around result)
    assert_eq!(cpu.reg[3], 0x0001_FFFF);
}

/// SUB between same registers (e.g., x1 = x1 - x1)
#[test]
fn sub_with_same_source_registers() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SUB x1, x1, x1 → x1 = 0
    let sub_instruction: Word = 0b0100000_00001_00001_000_00001_0110011;
    mem.store_word(0x0, sub_instruction);

    cpu.reg[1] = 123;

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.reg[1], 0);
}

/// SUB when all registers are zero
#[test]
fn sub_all_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SUB x31, x0, x0 → 0 - 0 = 0
    let sub_instruction: Word = 0b0100000_00000_00000_000_11111_0110011;
    mem.store_word(0x0, sub_instruction);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.reg[31], 0);
}
