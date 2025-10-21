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

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

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

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[31], 0);
}

/// Test that an SLT instruction can be fetched correctly
#[test]
fn slt_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: SLT x1, x2, x3
    // funct7=0000000, rs2=00011, rs1=00010, funct3=010, rd=00001, opcode=0110011
    let slt_instruction: Word = 0b0000000_00011_00010_010_00001_0110011;
    mem.store_word(0x0, slt_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, slt_instruction);
}

/// Basic SLT operation test (less than → true)
#[test]
fn slt_basic_less_than() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x1, x2, x3 → x1 = (x2 < x3) ? 1 : 0
    let slt_instruction: Word = 0b0000000_00011_00010_010_00001_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.reg[2] = 5; // x2
    cpu.reg[3] = 7; // x3

    cpu.clock_cycle(&mut mem);

    // 5 < 7 → x1 = 1
    assert_eq!(cpu.reg[1], 1);
}

/// SLT where rs1 > rs2 (false)
#[test]
fn slt_greater_than() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x4, x5, x6
    let slt_instruction: Word = 0b0000000_00110_00101_010_00100_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.reg[5] = 9;
    cpu.reg[6] = 3;

    cpu.clock_cycle(&mut mem);

    // 9 < 3 → false → 0
    assert_eq!(cpu.reg[4], 0);
}

/// SLT with equal operands
#[test]
fn slt_equal_operands() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x7, x8, x9
    let slt_instruction: Word = 0b0000000_01001_01000_010_00111_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.reg[8] = 42;
    cpu.reg[9] = 42;

    cpu.clock_cycle(&mut mem);

    // 42 < 42 → false
    assert_eq!(cpu.reg[7], 0);
}

/// SLT with signed negative operands
#[test]
fn slt_with_negative_operands() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x10, x11, x12
    let slt_instruction: Word = 0b0000000_01100_01011_010_01010_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.reg[11] = (-5i32) as u32; // x11 = -5
    cpu.reg[12] = 3; // x12 = 3

    cpu.clock_cycle(&mut mem);

    // -5 < 3 → true
    assert_eq!(cpu.reg[10], 1);
}

/// SLT with both negative operands
#[test]
fn slt_two_negative_numbers() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x13, x14, x15
    let slt_instruction: Word = 0b0000000_01111_01110_010_01101_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.reg[14] = (-10i32) as u32; // x14 = -10
    cpu.reg[15] = (-3i32) as u32; // x15 = -3

    cpu.clock_cycle(&mut mem);

    // -10 < -3 → true
    assert_eq!(cpu.reg[13], 1);
}

/// SLT where rs1 negative and rs2 positive (should be true)
#[test]
fn slt_negative_vs_positive() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x16, x17, x18
    let slt_instruction: Word = 0b0000000_10010_10001_010_10000_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.reg[17] = (-1i32) as u32;
    cpu.reg[18] = 1;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[16], 1);
}

/// SLT where rs1 positive and rs2 negative (should be false)
#[test]
fn slt_positive_vs_negative() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x19, x20, x21
    let slt_instruction: Word = 0b0000000_10101_10100_010_10011_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.reg[20] = 5;
    cpu.reg[21] = (-5i32) as u32;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[19], 0);
}

/// SLT when rs1 = 0 and rs2 positive
#[test]
fn slt_zero_vs_positive() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x1, x0, x2 → x1 = (0 < x2)
    let slt_instruction: Word = 0b0000000_00010_00000_010_00001_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.reg[2] = 123;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 1);
}

/// SLT writing to x0 should have no effect
#[test]
fn slt_write_to_x0_is_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x0, x1, x2
    let slt_instruction: Word = 0b0000000_00010_00001_010_00000_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.reg[1] = 10;
    cpu.reg[2] = 20;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[0], 0);
}

/// SLT where rs1 == rs2 (should be 0)
#[test]
fn slt_same_registers() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x3, x3, x3
    let slt_instruction: Word = 0b0000000_00011_00011_010_00011_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.reg[3] = 123;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[3], 0);
}

/// SLT with large values (unsigned wrap-around shouldn't matter for signed comparison)
#[test]
fn slt_large_values_signed_compare() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x5, x6, x7
    let slt_instruction: Word = 0b0000000_00111_00110_010_00101_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.reg[6] = 0x8000_0000; // -2147483648 in signed
    cpu.reg[7] = 0x7FFF_FFFF; //  2147483647 in signed

    cpu.clock_cycle(&mut mem);

    // -2147483648 < 2147483647 → true
    assert_eq!(cpu.reg[5], 1);
}

/// SLT when all registers are zero
#[test]
fn slt_all_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLT x31, x0, x0 → 0 < 0 = false
    let slt_instruction: Word = 0b0000000_00000_00000_010_11111_0110011;
    mem.store_word(0x0, slt_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[31], 0);
}

/// Test that an SLTU instruction can be fetched correctly
#[test]
fn sltu_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: SLTU x1, x2, x3
    // funct7=0000000, rs2=00011, rs1=00010, funct3=011, rd=00001, opcode=0110011
    let sltu_instruction: Word = 0b0000000_00011_00010_011_00001_0110011;
    mem.store_word(0x0, sltu_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, sltu_instruction);
}

/// Basic SLTU operation test (less than → true)
#[test]
fn sltu_basic_less_than() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x1, x2, x3 → x1 = (x2 < x3) ? 1 : 0
    let sltu_instruction: Word = 0b0000000_00011_00010_011_00001_0110011;
    mem.store_word(0x0, sltu_instruction);

    cpu.reg[2] = 5;
    cpu.reg[3] = 7;

    cpu.clock_cycle(&mut mem);

    // 5 < 7 → true
    assert_eq!(cpu.reg[1], 1);
}

/// SLTU where rs1 > rs2 (false)
#[test]
fn sltu_greater_than() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x4, x5, x6
    let sltu_instruction: Word = 0b0000000_00110_00101_011_00100_0110011;
    mem.store_word(0x0, sltu_instruction);

    cpu.reg[5] = 9;
    cpu.reg[6] = 3;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[4], 0);
}

/// SLTU with equal operands
#[test]
fn sltu_equal_operands() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x7, x8, x9
    let sltu_instruction: Word = 0b0000000_01001_01000_011_00111_0110011;
    mem.store_word(0x0, sltu_instruction);

    cpu.reg[8] = 42;
    cpu.reg[9] = 42;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[7], 0);
}

/// SLTU with signed negative values (interpreted as large unsigned)
#[test]
fn sltu_signed_negative_operands() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x10, x11, x12
    let sltu_instruction: Word = 0b0000000_01100_01011_011_01010_0110011;
    mem.store_word(0x0, sltu_instruction);

    // x11 = -1 (0xFFFF_FFFF), x12 = 3
    cpu.reg[11] = (-1i32) as u32;
    cpu.reg[12] = 3;

    cpu.clock_cycle(&mut mem);

    // Unsigned compare: 0xFFFF_FFFF < 3 → false
    assert_eq!(cpu.reg[10], 0);
}

/// SLTU where rs1 small unsigned, rs2 large unsigned
#[test]
fn sltu_unsigned_less_than_large_value() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x13, x14, x15
    let sltu_instruction: Word = 0b0000000_01111_01110_011_01101_0110011;
    mem.store_word(0x0, sltu_instruction);

    cpu.reg[14] = 1;
    cpu.reg[15] = 0xFFFF_FFFF;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[13], 1);
}

/// SLTU with rs1 = 0 and rs2 > 0
#[test]
fn sltu_zero_vs_positive() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x1, x0, x2
    let sltu_instruction: Word = 0b0000000_00010_00000_011_00001_0110011;
    mem.store_word(0x0, sltu_instruction);

    cpu.reg[2] = 123;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 1);
}

/// SLTU with rs1 = x2, rs2 = x0 (positive vs 0)
#[test]
fn sltu_positive_vs_zero() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x3, x2, x0
    let sltu_instruction: Word = 0b0000000_00000_00010_011_00011_0110011;
    mem.store_word(0x0, sltu_instruction);

    cpu.reg[2] = 50;

    cpu.clock_cycle(&mut mem);

    // 50 < 0 → false
    assert_eq!(cpu.reg[3], 0);
}

/// SLTU write to x0 (should not modify x0)
#[test]
fn sltu_write_to_x0_is_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x0, x1, x2
    let sltu_instruction: Word = 0b0000000_00010_00001_011_00000_0110011;
    mem.store_word(0x0, sltu_instruction);

    cpu.reg[1] = 1;
    cpu.reg[2] = 2;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[0], 0);
}

/// SLTU with same source registers (should be false)
#[test]
fn sltu_same_registers() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x5, x5, x5
    let sltu_instruction: Word = 0b0000000_00101_00101_011_00101_0110011;
    mem.store_word(0x0, sltu_instruction);

    cpu.reg[5] = 999;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[5], 0);
}

/// SLTU large unsigned comparison (0x7FFFFFFF vs 0x80000000)
#[test]
fn sltu_large_unsigned_compare() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x6, x7, x8
    let sltu_instruction: Word = 0b0000000_01000_00111_011_00110_0110011;
    mem.store_word(0x0, sltu_instruction);

    cpu.reg[7] = 0x7FFF_FFFF;
    cpu.reg[8] = 0x8000_0000;

    cpu.clock_cycle(&mut mem);

    // Unsigned: 0x7FFFFFFF < 0x80000000 → true
    assert_eq!(cpu.reg[6], 1);
}

/// SLTU large reversed unsigned comparison (0x80000000 vs 0x7FFFFFFF)
#[test]
fn sltu_large_reversed_unsigned_compare() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x9, x10, x11
    let sltu_instruction: Word = 0b0000000_01011_01010_011_01001_0110011;
    mem.store_word(0x0, sltu_instruction);

    cpu.reg[10] = 0x8000_0000;
    cpu.reg[11] = 0x7FFF_FFFF;

    cpu.clock_cycle(&mut mem);

    // Unsigned: 0x80000000 < 0x7FFFFFFF → false
    assert_eq!(cpu.reg[9], 0);
}

/// SLTU when all registers are zero
#[test]
fn sltu_all_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTU x31, x0, x0 → 0 < 0 = false
    let sltu_instruction: Word = 0b0000000_00000_00000_011_11111_0110011;
    mem.store_word(0x0, sltu_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[31], 0);
}

/// Test that an AND instruction can be fetched correctly
#[test]
fn and_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: AND x1, x2, x3
    // funct7=0000000, rs2=00011, rs1=00010, funct3=111, rd=00001, opcode=0110011
    let and_instruction: Word = 0b0000000_00011_00010_111_00001_0110011;
    mem.store_word(0x0, and_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, and_instruction);
}

/// Basic AND operation test
#[test]
fn and_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // AND x1, x2, x3
    let and_instruction: Word = 0b0000000_00011_00010_111_00001_0110011;
    mem.store_word(0x0, and_instruction);

    cpu.reg[2] = 0b1100; // x2 = 12
    cpu.reg[3] = 0b1010; // x3 = 10

    cpu.clock_cycle(&mut mem);

    // 1100 & 1010 = 1000 (8)
    assert_eq!(cpu.reg[1], 0b1000);
}

/// AND with all bits set
#[test]
fn and_with_all_bits_set() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // AND x4, x5, x6
    let and_instruction: Word = 0b0000000_00110_00101_111_00100_0110011;
    mem.store_word(0x0, and_instruction);

    cpu.reg[5] = 0xFFFF_FFFF;
    cpu.reg[6] = 0x1234_5678;

    cpu.clock_cycle(&mut mem);

    // 0xFFFF_FFFF & 0x12345678 = 0x12345678
    assert_eq!(cpu.reg[4], 0x1234_5678);
}

/// AND with zeros
#[test]
fn and_with_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // AND x7, x8, x9
    let and_instruction: Word = 0b0000000_01001_01000_111_00111_0110011;
    mem.store_word(0x0, and_instruction);

    cpu.reg[8] = 0xFFFF_FFFF;
    cpu.reg[9] = 0x0000_0000;

    cpu.clock_cycle(&mut mem);

    // anything & 0 = 0
    assert_eq!(cpu.reg[7], 0);
}

/// AND where both operands are the same
#[test]
fn and_same_operands() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // AND x10, x11, x11
    let and_instruction: Word = 0b0000000_01011_01011_111_01010_0110011;
    mem.store_word(0x0, and_instruction);

    cpu.reg[11] = 0xDEAD_BEEF;

    cpu.clock_cycle(&mut mem);

    // x11 & x11 = x11
    assert_eq!(cpu.reg[10], 0xDEAD_BEEF);
}

/// AND with alternating bit patterns
#[test]
fn and_alternating_bit_patterns() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // AND x12, x13, x14
    let and_instruction: Word = 0b0000000_01110_01101_111_01100_0110011;
    mem.store_word(0x0, and_instruction);

    cpu.reg[13] = 0xAAAA_AAAA; // 1010...
    cpu.reg[14] = 0x5555_5555; // 0101...

    cpu.clock_cycle(&mut mem);

    // 1010... & 0101... = 0000...
    assert_eq!(cpu.reg[12], 0x0000_0000);
}

/// AND where one operand is x0 (always zero)
#[test]
fn and_with_x0_operand() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // AND x1, x0, x2 → x1 = 0 & x2 = 0
    let and_instruction: Word = 0b0000000_00010_00000_111_00001_0110011;
    mem.store_word(0x0, and_instruction);

    cpu.reg[2] = 0xFFFF_FFFF;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 0);
}

/// AND with destination register x0 (should not modify x0)
#[test]
fn and_write_to_x0_is_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // AND x0, x1, x2
    let and_instruction: Word = 0b0000000_00010_00001_111_00000_0110011;
    mem.store_word(0x0, and_instruction);

    cpu.reg[1] = 0xAAAA_AAAA;
    cpu.reg[2] = 0x5555_5555;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[0], 0);
}

/// AND with large random values
#[test]
fn and_large_values() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // AND x3, x4, x5
    let and_instruction: Word = 0b0000000_00101_00100_111_00011_0110011;
    mem.store_word(0x0, and_instruction);

    cpu.reg[4] = 0x1234_ABCD;
    cpu.reg[5] = 0xFFFF_00FF;

    cpu.clock_cycle(&mut mem);

    // 0x1234ABCD & 0xFFFF00FF = 0x123400CD
    assert_eq!(cpu.reg[3], 0x1234_00CD);
}

/// AND where all registers are zero
#[test]
fn and_all_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // AND x31, x0, x0 → 0 & 0 = 0
    let and_instruction: Word = 0b0000000_00000_00000_111_11111_0110011;
    mem.store_word(0x0, and_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[31], 0);
}

/// Test that an OR instruction can be fetched correctly
#[test]
fn or_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: OR x1, x2, x3
    // funct7=0000000, rs2=00011, rs1=00010, funct3=110, rd=00001, opcode=0110011
    let or_instruction: Word = 0b0000000_00011_00010_110_00001_0110011;
    mem.store_word(0x0, or_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, or_instruction);
}

/// Basic OR operation test
#[test]
fn or_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // OR x1, x2, x3
    let or_instruction: Word = 0b0000000_00011_00010_110_00001_0110011;
    mem.store_word(0x0, or_instruction);

    cpu.reg[2] = 0b1100; // 12
    cpu.reg[3] = 0b1010; // 10

    cpu.clock_cycle(&mut mem);

    // 1100 | 1010 = 1110 (14)
    assert_eq!(cpu.reg[1], 0b1110);
}

/// OR with all bits set
#[test]
fn or_with_all_bits_set() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // OR x4, x5, x6
    let or_instruction: Word = 0b0000000_00110_00101_110_00100_0110011;
    mem.store_word(0x0, or_instruction);

    cpu.reg[5] = 0xFFFF_FFFF;
    cpu.reg[6] = 0x1234_5678;

    cpu.clock_cycle(&mut mem);

    // 0xFFFF_FFFF | anything = 0xFFFF_FFFF
    assert_eq!(cpu.reg[4], 0xFFFF_FFFF);
}

/// OR with zeros
#[test]
fn or_with_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // OR x7, x8, x9
    let or_instruction: Word = 0b0000000_01001_01000_110_00111_0110011;
    mem.store_word(0x0, or_instruction);

    cpu.reg[8] = 0xFFFF_0000;
    cpu.reg[9] = 0x0000_0000;

    cpu.clock_cycle(&mut mem);

    // x8 | 0 = x8
    assert_eq!(cpu.reg[7], 0xFFFF_0000);
}

/// OR where both operands are the same
#[test]
fn or_same_operands() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // OR x10, x11, x11
    let or_instruction: Word = 0b0000000_01011_01011_110_01010_0110011;
    mem.store_word(0x0, or_instruction);

    cpu.reg[11] = 0xDEAD_BEEF;

    cpu.clock_cycle(&mut mem);

    // x11 | x11 = x11
    assert_eq!(cpu.reg[10], 0xDEAD_BEEF);
}

/// OR with alternating bit patterns
#[test]
fn or_alternating_bit_patterns() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // OR x12, x13, x14
    let or_instruction: Word = 0b0000000_01110_01101_110_01100_0110011;
    mem.store_word(0x0, or_instruction);

    cpu.reg[13] = 0xAAAA_AAAA; // 1010...
    cpu.reg[14] = 0x5555_5555; // 0101...

    cpu.clock_cycle(&mut mem);

    // 1010... | 0101... = 1111...
    assert_eq!(cpu.reg[12], 0xFFFF_FFFF);
}

/// OR where one operand is x0 (always zero)
#[test]
fn or_with_x0_operand() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // OR x1, x0, x2 → x1 = 0 | x2 = x2
    let or_instruction: Word = 0b0000000_00010_00000_110_00001_0110011;
    mem.store_word(0x0, or_instruction);

    cpu.reg[2] = 0x1234_5678;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 0x1234_5678);
}

/// OR with destination register x0 (should not modify x0)
#[test]
fn or_write_to_x0_is_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // OR x0, x1, x2
    let or_instruction: Word = 0b0000000_00010_00001_110_00000_0110011;
    mem.store_word(0x0, or_instruction);

    cpu.reg[1] = 0xAAAA_AAAA;
    cpu.reg[2] = 0x5555_5555;

    cpu.clock_cycle(&mut mem);

    // x0 is hardwired to zero
    assert_eq!(cpu.reg[0], 0);
}

/// OR with large random values
#[test]
fn or_large_values() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // OR x3, x4, x5
    let or_instruction: Word = 0b0000000_00101_00100_110_00011_0110011;
    mem.store_word(0x0, or_instruction);

    cpu.reg[4] = 0x1234_0000;
    cpu.reg[5] = 0x0000_ABCD;

    cpu.clock_cycle(&mut mem);

    // 0x12340000 | 0x0000ABCD = 0x1234ABCD
    assert_eq!(cpu.reg[3], 0x1234_ABCD);
}

/// OR when all registers are zero
#[test]
fn or_all_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // OR x31, x0, x0 → 0 | 0 = 0
    let or_instruction: Word = 0b0000000_00000_00000_110_11111_0110011;
    mem.store_word(0x0, or_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[31], 0);
}

/// Test that a XOR instruction can be fetched correctly
#[test]
fn xor_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: XOR x1, x2, x3
    // funct7=0000000, rs2=00011, rs1=00010, funct3=100, rd=00001, opcode=0110011
    let xor_instruction: Word = 0b0000000_00011_00010_100_00001_0110011;
    mem.store_word(0x0, xor_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, xor_instruction);
}

/// Basic XOR operation test
#[test]
fn xor_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XOR x1, x2, x3
    let xor_instruction: Word = 0b0000000_00011_00010_100_00001_0110011;
    mem.store_word(0x0, xor_instruction);

    cpu.reg[2] = 0b1100; // 12
    cpu.reg[3] = 0b1010; // 10

    cpu.clock_cycle(&mut mem);

    // 1100 ^ 1010 = 0110 (6)
    assert_eq!(cpu.reg[1], 0b0110);
}

/// XOR with identical operands
#[test]
fn xor_same_operands() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XOR x4, x5, x5
    let xor_instruction: Word = 0b0000000_00101_00101_100_00100_0110011;
    mem.store_word(0x0, xor_instruction);

    cpu.reg[5] = 0xFFFF_FFFF;

    cpu.clock_cycle(&mut mem);

    // a ^ a = 0
    assert_eq!(cpu.reg[4], 0);
}

/// XOR with all bits set
#[test]
fn xor_with_all_bits_set() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XOR x6, x7, x8
    let xor_instruction: Word = 0b0000000_01000_00111_100_00110_0110011;
    mem.store_word(0x0, xor_instruction);

    cpu.reg[7] = 0xFFFF_FFFF;
    cpu.reg[8] = 0x1234_5678;

    cpu.clock_cycle(&mut mem);

    // 0xFFFF_FFFF ^ 0x12345678 = bitwise NOT of 0x12345678
    assert_eq!(cpu.reg[6], !0x1234_5678);
}

/// XOR with zeros
#[test]
fn xor_with_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XOR x9, x10, x11
    let xor_instruction: Word = 0b0000000_01011_01010_100_01001_0110011;
    mem.store_word(0x0, xor_instruction);

    cpu.reg[10] = 0x0000_0000;
    cpu.reg[11] = 0xABCD_1234;

    cpu.clock_cycle(&mut mem);

    // 0 ^ a = a
    assert_eq!(cpu.reg[9], 0xABCD_1234);
}

/// XOR alternating bit patterns
#[test]
fn xor_alternating_bit_patterns() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XOR x12, x13, x14
    let xor_instruction: Word = 0b0000000_01110_01101_100_01100_0110011;
    mem.store_word(0x0, xor_instruction);

    cpu.reg[13] = 0xAAAA_AAAA; // 1010...
    cpu.reg[14] = 0x5555_5555; // 0101...

    cpu.clock_cycle(&mut mem);

    // 1010... ^ 0101... = 1111...
    assert_eq!(cpu.reg[12], 0xFFFF_FFFF);
}

/// XOR where one operand is x0 (zero)
#[test]
fn xor_with_x0_operand() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XOR x1, x0, x2 → x1 = 0 ^ x2 = x2
    let xor_instruction: Word = 0b0000000_00010_00000_100_00001_0110011;
    mem.store_word(0x0, xor_instruction);

    cpu.reg[2] = 0x1234_5678;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 0x1234_5678);
}

/// XOR with destination register x0 (should not modify x0)
#[test]
fn xor_write_to_x0_is_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XOR x0, x1, x2
    let xor_instruction: Word = 0b0000000_00010_00001_100_00000_0110011;
    mem.store_word(0x0, xor_instruction);

    cpu.reg[1] = 0xAAAA_AAAA;
    cpu.reg[2] = 0x5555_5555;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[0], 0);
}

/// XOR with large values
#[test]
fn xor_large_values() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XOR x3, x4, x5
    let xor_instruction: Word = 0b0000000_00101_00100_100_00011_0110011;
    mem.store_word(0x0, xor_instruction);

    cpu.reg[4] = 0x1234_ABCD;
    cpu.reg[5] = 0xFFFF_00FF;

    cpu.clock_cycle(&mut mem);

    // 0x1234ABCD ^ 0xFFFF00FF = 0xEDCB_AB32
    assert_eq!(cpu.reg[3], 0xEDCB_AB32);
}

/// XOR when all registers are zero
#[test]
fn xor_all_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XOR x31, x0, x0 → 0 ^ 0 = 0
    let xor_instruction: Word = 0b0000000_00000_00000_100_11111_0110011;
    mem.store_word(0x0, xor_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[31], 0);
}

/// Test that an SLL instruction can be fetched correctly
#[test]
fn sll_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: SLL x1, x2, x3
    // funct7=0000000, rs2=00011, rs1=00010, funct3=001, rd=00001, opcode=0110011
    let sll_instruction: Word = 0b0000000_00011_00010_001_00001_0110011;
    mem.store_word(0x0, sll_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, sll_instruction);
}

/// Basic SLL operation test
#[test]
fn sll_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLL x1, x2, x3 → x1 = x2 << (x3 & 0x1F)
    let sll_instruction: Word = 0b0000000_00011_00010_001_00001_0110011;
    mem.store_word(0x0, sll_instruction);

    cpu.reg[2] = 0b0001; // 1
    cpu.reg[3] = 2; // shift by 2 bits

    cpu.clock_cycle(&mut mem);

    // 1 << 2 = 4
    assert_eq!(cpu.reg[1], 0b0100);
}

/// SLL shift by zero (no change)
#[test]
fn sll_shift_by_zero() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLL x4, x5, x6
    let sll_instruction: Word = 0b0000000_00110_00101_001_00100_0110011;
    mem.store_word(0x0, sll_instruction);

    cpu.reg[5] = 0x1234_5678;
    cpu.reg[6] = 0; // shift by 0

    cpu.clock_cycle(&mut mem);

    // No change expected
    assert_eq!(cpu.reg[4], 0x1234_5678);
}

/// SLL shift by 31 (maximum shift in RV32)
#[test]
fn sll_shift_by_31() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLL x7, x8, x9
    let sll_instruction: Word = 0b0000000_01001_01000_001_00111_0110011;
    mem.store_word(0x0, sll_instruction);

    cpu.reg[8] = 0x1;
    cpu.reg[9] = 31;

    cpu.clock_cycle(&mut mem);

    // 1 << 31 = 0x8000_0000
    assert_eq!(cpu.reg[7], 0x8000_0000);
}

/// SLL shift amount greater than 31 (should wrap around using lower 5 bits)
#[test]
fn sll_shift_by_more_than_31() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLL x10, x11, x12
    let sll_instruction: Word = 0b0000000_01100_01011_001_01010_0110011;
    mem.store_word(0x0, sll_instruction);

    cpu.reg[11] = 0x1;
    cpu.reg[12] = 35; // 35 & 0x1F = 3

    cpu.clock_cycle(&mut mem);

    // 1 << 3 = 8
    assert_eq!(cpu.reg[10], 0x8);
}

/// SLL with alternating bit pattern
#[test]
fn sll_alternating_bits() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLL x13, x14, x15
    let sll_instruction: Word = 0b0000000_01111_01110_001_01101_0110011;
    mem.store_word(0x0, sll_instruction);

    cpu.reg[14] = 0xAAAA_AAAA; // 1010...
    cpu.reg[15] = 1; // shift left by 1

    cpu.clock_cycle(&mut mem);

    // Shift left by 1, low bit becomes 0
    assert_eq!(cpu.reg[13], 0x5555_5554);
}

/// SLL with zero as source (x2 = 0)
#[test]
fn sll_with_zero_source() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLL x16, x0, x17
    let sll_instruction: Word = 0b0000000_10001_00000_001_10000_0110011;
    mem.store_word(0x0, sll_instruction);

    cpu.reg[17] = 10;

    cpu.clock_cycle(&mut mem);

    // 0 << anything = 0
    assert_eq!(cpu.reg[16], 0);
}

/// SLL with destination register x0 (should not modify x0)
#[test]
fn sll_write_to_x0_is_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLL x0, x1, x2
    let sll_instruction: Word = 0b0000000_00010_00001_001_00000_0110011;
    mem.store_word(0x0, sll_instruction);

    cpu.reg[1] = 0xF0F0_F0F0;
    cpu.reg[2] = 4;

    cpu.clock_cycle(&mut mem);

    // x0 remains 0
    assert_eq!(cpu.reg[0], 0);
}

/// SLL with large initial value
#[test]
fn sll_large_value_shift() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLL x3, x4, x5
    let sll_instruction: Word = 0b0000000_00101_00100_001_00011_0110011;
    mem.store_word(0x0, sll_instruction);

    cpu.reg[4] = 0x000F_FFFF;
    cpu.reg[5] = 8;

    cpu.clock_cycle(&mut mem);

    // 0x000FFFFF << 8 = 0x0FFFFF00
    assert_eq!(cpu.reg[3], 0x0FFF_FF00);
}

/// SLL when all registers are zero
#[test]
fn sll_all_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLL x31, x0, x0 → 0 << 0 = 0
    let sll_instruction: Word = 0b0000000_00000_00000_001_11111_0110011;
    mem.store_word(0x0, sll_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[31], 0);
}

/// Test that an SRL instruction can be fetched correctly
#[test]
fn srl_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: SRL x1, x2, x3
    // funct7=0000000, rs2=00011, rs1=00010, funct3=101, rd=00001, opcode=0110011
    let srl_instruction: Word = 0b0000000_00011_00010_101_00001_0110011;
    mem.store_word(0x0, srl_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, srl_instruction);
}

/// Basic SRL operation test
#[test]
fn srl_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRL x1, x2, x3 → x1 = x2 >> (x3 & 0x1F)
    let srl_instruction: Word = 0b0000000_00011_00010_101_00001_0110011;
    mem.store_word(0x0, srl_instruction);

    cpu.reg[2] = 0b1000; // 8
    cpu.reg[3] = 2; // shift by 2

    cpu.clock_cycle(&mut mem);

    // 8 >> 2 = 2
    assert_eq!(cpu.reg[1], 0b10);
}

/// SRL shift by zero (no change)
#[test]
fn srl_shift_by_zero() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRL x4, x5, x6
    let srl_instruction: Word = 0b0000000_00110_00101_101_00100_0110011;
    mem.store_word(0x0, srl_instruction);

    cpu.reg[5] = 0xF0F0_F0F0;
    cpu.reg[6] = 0; // shift by 0

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[4], 0xF0F0_F0F0);
}

/// SRL shift by 31 (maximum in RV32)
#[test]
fn srl_shift_by_31() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRL x7, x8, x9
    let srl_instruction: Word = 0b0000000_01001_01000_101_00111_0110011;
    mem.store_word(0x0, srl_instruction);

    cpu.reg[8] = 0x8000_0000;
    cpu.reg[9] = 31;

    cpu.clock_cycle(&mut mem);

    // Logical right shift: only bit 31 moves to bit 0
    assert_eq!(cpu.reg[7], 1);
}

/// SRL shift by more than 31 (wraps via 0x1F)
#[test]
fn srl_shift_by_more_than_31() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRL x10, x11, x12
    let srl_instruction: Word = 0b0000000_01100_01011_101_01010_0110011;
    mem.store_word(0x0, srl_instruction);

    cpu.reg[11] = 0xFFFF_FFFF;
    cpu.reg[12] = 35; // 35 & 0x1F = 3

    cpu.clock_cycle(&mut mem);

    // 0xFFFF_FFFF >> 3 = 0x1FFF_FFFF
    assert_eq!(cpu.reg[10], 0x1FFF_FFFF);
}

/// SRL with alternating bit pattern
#[test]
fn srl_alternating_bits() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRL x13, x14, x15
    let srl_instruction: Word = 0b0000000_01111_01110_101_01101_0110011;
    mem.store_word(0x0, srl_instruction);

    cpu.reg[14] = 0xAAAA_AAAA; // 1010...
    cpu.reg[15] = 1;

    cpu.clock_cycle(&mut mem);

    // Logical shift right by 1
    assert_eq!(cpu.reg[13], 0x5555_5555);
}

/// SRL with zero source
#[test]
fn srl_with_zero_source() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRL x16, x0, x17
    let srl_instruction: Word = 0b0000000_10001_00000_101_10000_0110011;
    mem.store_word(0x0, srl_instruction);

    cpu.reg[17] = 10;

    cpu.clock_cycle(&mut mem);

    // 0 >> anything = 0
    assert_eq!(cpu.reg[16], 0);
}

/// SRL write to x0 should be ignored
#[test]
fn srl_write_to_x0_is_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRL x0, x1, x2
    let srl_instruction: Word = 0b0000000_00010_00001_101_00000_0110011;
    mem.store_word(0x0, srl_instruction);

    cpu.reg[1] = 0xFFFF_FFFF;
    cpu.reg[2] = 4;

    cpu.clock_cycle(&mut mem);

    // x0 must stay zero
    assert_eq!(cpu.reg[0], 0);
}

/// SRL with large initial value
#[test]
fn srl_large_value_shift() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRL x3, x4, x5
    let srl_instruction: Word = 0b0000000_00101_00100_101_00011_0110011;
    mem.store_word(0x0, srl_instruction);

    cpu.reg[4] = 0xFFFF_0000;
    cpu.reg[5] = 8;

    cpu.clock_cycle(&mut mem);

    // 0xFFFF_0000 >> 8 = 0x00FF_FF00
    assert_eq!(cpu.reg[3], 0x00FF_FF00);
}

/// SRL when all registers are zero
#[test]
fn srl_all_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRL x31, x0, x0 → 0 >> 0 = 0
    let srl_instruction: Word = 0b0000000_00000_00000_101_11111_0110011;
    mem.store_word(0x0, srl_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[31], 0);
}

/// SRL with same register as source and destination
#[test]
fn srl_same_registers() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRL x5, x5, x5 (x5 = x5 >> (x5 & 0x1F))
    let srl_instruction: Word = 0b0000000_00101_00101_101_00101_0110011;
    mem.store_word(0x0, srl_instruction);

    cpu.reg[5] = 0xF0F0_F0F0; // value = F0F0_F0F0 (shift amount = 0x10 = 16)
    cpu.clock_cycle(&mut mem);

    // 0xF0F0_F0F0 >> 16 = 0x0000_F0F0
    assert_eq!(cpu.reg[5], 0x0000_F0F0);
}

/// Test that an SRA instruction can be fetched correctly
#[test]
fn sra_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: SRA x1, x2, x3
    // funct7=0100000, rs2=00011, rs1=00010, funct3=101, rd=00001, opcode=0110011
    let sra_instruction: Word = 0b0100000_00011_00010_101_00001_0110011;
    mem.store_word(0x0, sra_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, sra_instruction);
}

/// Basic SRA operation test (positive number)
#[test]
fn sra_basic_operation_positive() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRA x1, x2, x3 → x1 = (x2 as i32) >> (x3 & 0x1F)
    let sra_instruction: Word = 0b0100000_00011_00010_101_00001_0110011;
    mem.store_word(0x0, sra_instruction);

    cpu.reg[2] = 0b1000; // 8
    cpu.reg[3] = 2; // shift by 2

    cpu.clock_cycle(&mut mem);

    // 8 >> 2 = 2
    assert_eq!(cpu.reg[1], 0b10);
}

/// SRA with negative number (sign extension)
#[test]
fn sra_negative_number_sign_extend() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRA x4, x5, x6
    let sra_instruction: Word = 0b0100000_00110_00101_101_00100_0110011;
    mem.store_word(0x0, sra_instruction);

    // x5 = -8 (0xFFFF_FFF8)
    cpu.reg[5] = (-8i32) as u32;
    cpu.reg[6] = 2;

    cpu.clock_cycle(&mut mem);

    // Arithmetic right shift keeps sign bits: -8 >> 2 = -2
    assert_eq!(cpu.reg[4] as i32, -2);
}

/// SRA shift by zero (no change)
#[test]
fn sra_shift_by_zero() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRA x7, x8, x9
    let sra_instruction: Word = 0b0100000_01001_01000_101_00111_0110011;
    mem.store_word(0x0, sra_instruction);

    cpu.reg[8] = 0xF000_0000;
    cpu.reg[9] = 0; // shift by 0

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[7], 0xF000_0000);
}

/// SRA shift by 31 (maximum shift)
#[test]
fn sra_shift_by_31() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRA x10, x11, x12
    let sra_instruction: Word = 0b0100000_01100_01011_101_01010_0110011;
    mem.store_word(0x0, sra_instruction);

    cpu.reg[11] = (-1i32) as u32; // 0xFFFF_FFFF
    cpu.reg[12] = 31;

    cpu.clock_cycle(&mut mem);

    // -1 >> 31 = -1 (sign bit stays 1)
    assert_eq!(cpu.reg[10] as i32, -1);
}

/// SRA shift by more than 31 (mask with 0x1F)
#[test]
fn sra_shift_by_more_than_31() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRA x13, x14, x15
    let sra_instruction: Word = 0b0100000_01111_01110_101_01101_0110011;
    mem.store_word(0x0, sra_instruction);

    cpu.reg[14] = (-128i32) as u32; // 0xFFFFFF80
    cpu.reg[15] = 35; // 35 & 0x1F = 3

    cpu.clock_cycle(&mut mem);

    // -128 >> 3 = -16
    assert_eq!(cpu.reg[13] as i32, -16);
}

/// SRA with alternating bits (negative)
#[test]
fn sra_alternating_bits_negative() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRA x19, x20, x21
    let sra_instruction: Word = 0b0100000_10101_10100_101_10011_0110011;
    mem.store_word(0x0, sra_instruction);

    cpu.reg[20] = 0xAAAA_AAAA; // signed = -1431655766
    cpu.reg[21] = 1;

    cpu.clock_cycle(&mut mem);

    // Arithmetic shift preserves sign bit (1)
    assert_eq!(cpu.reg[19], 0xD555_5555);
}

/// SRA with zero source (x0)
#[test]
fn sra_with_zero_source() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRA x22, x0, x23
    let sra_instruction: Word = 0b0100000_10111_00000_101_10110_0110011;
    mem.store_word(0x0, sra_instruction);

    cpu.reg[23] = 10;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[22], 0);
}

/// SRA write to x0 should be ignored
#[test]
fn sra_write_to_x0_is_ignored() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRA x0, x1, x2
    let sra_instruction: Word = 0b0100000_00010_00001_101_00000_0110011;
    mem.store_word(0x0, sra_instruction);

    cpu.reg[1] = (-1i32) as u32;
    cpu.reg[2] = 4;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[0], 0);
}

/// SRA with same register as all operands (x5 = x5 >> (x5 & 0x1F))
#[test]
fn sra_same_registers() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRA x5, x5, x5
    let sra_instruction: Word = 0b0100000_00101_00101_101_00101_0110011;
    mem.store_word(0x0, sra_instruction);

    cpu.reg[5] = (-256i32) as u32; // 0xFFFF_FF00 → shift by 0

    cpu.clock_cycle(&mut mem);

    // -256 >> 16 = -1 (0xFFFF_FFFF)
    assert_eq!(cpu.reg[5] as i32, -256);
}

/// SRA when all registers are zero
#[test]
fn sra_all_zeros() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRA x31, x0, x0 → 0 >> 0 = 0
    let sra_instruction: Word = 0b0100000_00000_00000_101_11111_0110011;
    mem.store_word(0x0, sra_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[31], 0);
}
