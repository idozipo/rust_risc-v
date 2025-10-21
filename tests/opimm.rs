use rust_risc_v::*;

#[test]
fn addi_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: ADDI x1, x0, 10 (opcode for OPIMM is 0b0010011)
    let addi_instruction: Word = 0b00000000001000000000000010010011; // This is the binary representation of the instruction
    mem.store_word(0x0, addi_instruction); // Store the instruction at address 0

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, addi_instruction); // The fetched instruction should match the stored one
}

#[test]
fn addi_regular_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: ADDI x1, x2, 2
    let addi_instruction: Word = 0b0000000_00010_00010_000_00001_0010011;
    mem.store_word(0x0, addi_instruction); // Store the instruction at address 0

    // set register x2 to 8
    cpu.reg[2] = 8;

    // Execute the instruction at 0x0
    cpu.clock_cycle(&mut mem);

    // After execution, register x1 should contain 10
    assert_eq!(cpu.reg[1], 10);
}

#[test]
fn addi_with_reg_0() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: ADDI x1, x0, 3 (like MOV x1, 3)
    let addi_instruction: Word = 0b0000000_00011_00000_000_00001_0010011;
    mem.store_word(0x0, addi_instruction); // Store the instruction at address 0

    // Execute the instruction at 0x0
    cpu.clock_cycle(&mut mem);

    // After execution, register x1 should contain 3
    assert_eq!(cpu.reg[1], 3);
}

#[test]
fn addi_with_all_zeros_reg_0() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: ADDI x31, x0, 0 (like MOV x31, 0)
    let addi_instruction: Word = 0b0000000_00000_00000_000_11111_0010011;
    mem.store_word(0x0, addi_instruction); // Store the instruction at address 0

    // Execute the instruction at 0x0
    cpu.clock_cycle(&mut mem);

    // After execution, register x31 should contain 0
    assert_eq!(cpu.reg[1], 0);
}

#[test]
fn addi_with_min_neg_imm_value() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: ADDI x2, x0, -2048
    let addi_instruction: Word = 0b1000000_00000_00000_000_00010_0010011;
    mem.store_word(0x0, addi_instruction); // Store the instruction at address 0

    // Execute the instruction at 0x0
    cpu.clock_cycle(&mut mem);

    // After execution, register x2 should contain -2048 (4294965248 as u32)
    assert_eq!(cpu.reg[2], 4294965248);
}

#[test]
fn addi_with_all_ones_in_reg() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: ADDI x2, x1, 2
    let addi_instruction: Word = 0b0000000_00010_00001_000_00010_0010011;
    mem.store_word(0x0, addi_instruction); // Store the instruction at address 0

    // set register x1 to all ones (u32::MAX)
    cpu.reg[1] = u32::MAX;

    // Execute the instruction at 0x0
    cpu.clock_cycle(&mut mem);

    // After execution, register x2 should contain 1 (wrapping around)
    assert_eq!(cpu.reg[2], 1);
}

#[test]
fn slti_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: SLTI x1, x0, 10 (opcode for OPIMM is 0b0010011, funct3 = 010)
    // Binary fields: imm=000000001010, rs1=00000, funct3=010, rd=00001, opcode=0010011
    let slti_instruction: Word = 0b000000001010_00000_010_00001_0010011;
    mem.store_word(0x0, slti_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, slti_instruction);
}

#[test]
fn slti_regular_less_than() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTI x1, x2, 10  -> if (x2 < 10) then x1 = 1 else x1 = 0
    let slti_instruction: Word = 0b000000001010_00010_010_00001_0010011;
    mem.store_word(0x0, slti_instruction);

    cpu.reg[2] = 5; // x2 = 5 < 10

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 1); // should set
}

#[test]
fn slti_regular_not_less_than() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTI x1, x2, 10
    let slti_instruction: Word = 0b000000001010_00010_010_00001_0010011;
    mem.store_word(0x0, slti_instruction);

    cpu.reg[2] = 20; // x2 = 20 >= 10

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 0);
}

#[test]
fn slti_with_negative_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTI x3, x4, -1 (imm=0xFFF, 12-bit two’s complement)
    let slti_instruction: Word = 0b111111111111_00100_010_00011_0010011;
    mem.store_word(0x0, slti_instruction);
    mem.store_word(0x4, slti_instruction);

    cpu.reg[4] = 0; // 0 < -1 ? false

    cpu.clock_cycle(&mut mem);
    assert_eq!(cpu.reg[3], 0);

    // Try another case: x4 = -2 (signed)
    cpu.reg[4] = (-2i32) as u32;
    cpu.clock_cycle(&mut mem);
    assert_eq!(cpu.reg[3], 1);
}

#[test]
fn slti_with_reg_0() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTI x1, x0, 5 → compare 0 < 5 → true → x1 = 1
    let slti_instruction: Word = 0b000000000101_00000_010_00001_0010011;
    mem.store_word(0x0, slti_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 1);
}

#[test]
fn slti_with_negative_register_value() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTI x5, x6, 1
    let slti_instruction: Word = 0b000000000001_00110_010_00101_0010011;
    mem.store_word(0x0, slti_instruction);

    // x6 = -10 (signed)
    cpu.reg[6] = (-10i32) as u32;

    cpu.clock_cycle(&mut mem);

    // -10 < 1 → true
    assert_eq!(cpu.reg[5], 1);
}

#[test]
fn slti_with_same_value_as_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTI x2, x3, 5
    let slti_instruction: Word = 0b000000000101_00011_010_00010_0010011;
    mem.store_word(0x0, slti_instruction);

    cpu.reg[3] = 5;

    cpu.clock_cycle(&mut mem);

    // 5 < 5 → false
    assert_eq!(cpu.reg[2], 0);
}

#[test]
fn sltiu_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: SLTIU x1, x0, 10 (opcode 0010011, funct3 = 011)
    // imm = 10, rs1 = 0, funct3 = 011, rd = 1, opcode = 0010011
    let sltiu_instruction: Word = 0b000000001010_00000_011_00001_0010011;
    mem.store_word(0x0, sltiu_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, sltiu_instruction);
}

#[test]
fn sltiu_regular_less_than_unsigned() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTIU x1, x2, 10
    let sltiu_instruction: Word = 0b000000001010_00010_011_00001_0010011;
    mem.store_word(0x0, sltiu_instruction);

    cpu.reg[2] = 5; // 5 < 10 (unsigned)

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 1);
}

#[test]
fn sltiu_regular_not_less_than_unsigned() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTIU x1, x2, 10
    let sltiu_instruction: Word = 0b000000001010_00010_011_00001_0010011;
    mem.store_word(0x0, sltiu_instruction);

    cpu.reg[2] = 20; // 20 >= 10

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 0);
}

#[test]
fn sltiu_with_negative_register_value_interpreted_unsigned() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTIU x3, x4, 10
    let sltiu_instruction: Word = 0b000000001010_00100_011_00011_0010011;
    mem.store_word(0x0, sltiu_instruction);

    // x4 = -1 signed, which is 0xFFFF_FFFF unsigned (large number)
    cpu.reg[4] = (-1i32) as u32;

    cpu.clock_cycle(&mut mem);

    // As unsigned: 0xFFFF_FFFF > 10 → false
    assert_eq!(cpu.reg[3], 0);
}

#[test]
fn sltiu_with_reg_0() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTIU x1, x0, 5 → compare 0 < 5 (unsigned) → true
    let sltiu_instruction: Word = 0b000000000101_00000_011_00001_0010011;
    mem.store_word(0x0, sltiu_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 1);
}

#[test]
fn sltiu_with_equal_value() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTIU x2, x3, 255
    let sltiu_instruction: Word = 0b000011111111_00011_011_00010_0010011;
    mem.store_word(0x0, sltiu_instruction);

    cpu.reg[3] = 255;

    cpu.clock_cycle(&mut mem);

    // 255 < 255 (unsigned) → false
    assert_eq!(cpu.reg[2], 0);
}

#[test]
fn sltiu_with_large_unsigned_comparison() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTIU x5, x6, 0xFFF
    let sltiu_instruction: Word = 0b111111111111_00110_011_00101_0010011;
    mem.store_word(0x0, sltiu_instruction);

    // x6 = 0xFFFF_FFFE
    cpu.reg[6] = 0xFFFF_FFFE;

    cpu.clock_cycle(&mut mem);

    // unsigned: 0xFFFF_FFFE < 0xFFFF_FFFF → true
    assert_eq!(cpu.reg[5], 1);
}

#[test]
fn sltiu_with_small_unsigned_register_value() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTIU x5, x6, 0xFFF
    let sltiu_instruction: Word = 0b111111111111_00110_011_00101_0010011;
    mem.store_word(0x0, sltiu_instruction);

    cpu.reg[6] = 100;

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[5], 1);
}

#[test]
fn andi_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: ANDI x1, x0, 0b1010 (rd=x1, rs1=x0, imm=10)
    let andi_instruction: Word = 0b000000001010_00000_111_00001_0010011;
    mem.store_word(0x0, andi_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, andi_instruction);
}

#[test]
fn andi_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ANDI x1, x2, 0b1100
    let andi_instruction: Word = 0b000000001100_00010_111_00001_0010011;
    mem.store_word(0x0, andi_instruction);

    cpu.reg[2] = 0b1010; // 10 decimal

    cpu.clock_cycle(&mut mem);

    // 0b1010 & 0b1100 = 0b1000 (8)
    assert_eq!(cpu.reg[1], 0b1000);
}

#[test]
fn andi_with_reg_0() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ANDI x1, x0, 0b1111
    let andi_instruction: Word = 0b000000001111_00000_111_00001_0010011;
    mem.store_word(0x0, andi_instruction);

    cpu.clock_cycle(&mut mem);

    // x0 = 0, so result = 0 & imm = 0
    assert_eq!(cpu.reg[1], 0);
}

#[test]
fn andi_with_negative_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ANDI x1, x2, -4 (0xFFF...C 12-bit sign-extended)
    let andi_instruction: Word = 0b111111111100_00010_111_00001_0010011;
    mem.store_word(0x0, andi_instruction);

    cpu.reg[2] = 0b1010; // 10 decimal

    cpu.clock_cycle(&mut mem);

    // -4 = 0xFFFF_FFFC, 0b1010 & 0xFFFF_FFFC = 0b1000 (8)
    assert_eq!(cpu.reg[1], 0b1000);
}

#[test]
fn andi_with_all_ones_register() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ANDI x3, x1, 0b1010
    let andi_instruction: Word = 0b000000001010_00001_111_00011_0010011;
    mem.store_word(0x0, andi_instruction);

    cpu.reg[1] = u32::MAX; // all ones

    cpu.clock_cycle(&mut mem);

    // u32::MAX & 0b1010 = 0b1010
    assert_eq!(cpu.reg[3], 0b1010);
}

#[test]
fn andi_with_zero_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ANDI x2, x1, 0
    let andi_instruction: Word = 0b000000000000_00001_111_00010_0010011;
    mem.store_word(0x0, andi_instruction);

    cpu.reg[1] = 0b1101;

    cpu.clock_cycle(&mut mem);

    // Any number & 0 = 0
    assert_eq!(cpu.reg[2], 0);
}

#[test]
fn andi_with_all_ones_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ANDI x2, x1, -1 (0xFFF sign-extended = 0xFFFF_FFFF)
    let andi_instruction: Word = 0b111111111111_00001_111_00010_0010011;
    mem.store_word(0x0, andi_instruction);

    cpu.reg[1] = 0b101010;

    cpu.clock_cycle(&mut mem);

    // x1 & -1 = x1
    assert_eq!(cpu.reg[2], 0b101010);
}

#[test]
fn ori_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: ORI x1, x0, 0b1010 (rd=x1, rs1=x0, imm=10)
    let ori_instruction: Word = 0b000000001010_00000_110_00001_0010011;
    mem.store_word(0x0, ori_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, ori_instruction);
}

#[test]
fn ori_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ORI x1, x2, 0b1100
    let ori_instruction: Word = 0b000000001100_00010_110_00001_0010011;
    mem.store_word(0x0, ori_instruction);

    cpu.reg[2] = 0b1010; // 10 decimal

    cpu.clock_cycle(&mut mem);

    // 0b1010 | 0b1100 = 0b1110 (14)
    assert_eq!(cpu.reg[1], 0b1110);
}

#[test]
fn ori_with_reg_0() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ORI x1, x0, 0b1111
    let ori_instruction: Word = 0b000000001111_00000_110_00001_0010011;
    mem.store_word(0x0, ori_instruction);

    cpu.clock_cycle(&mut mem);

    // x0 = 0, so result = 0 | imm = 0b1111
    assert_eq!(cpu.reg[1], 0b1111);
}

#[test]
fn ori_with_negative_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ORI x1, x2, -4 (0xFFF...C 12-bit sign-extended)
    let ori_instruction: Word = 0b111111111100_00010_110_00001_0010011;
    mem.store_word(0x0, ori_instruction);

    cpu.reg[2] = 0b1010; // 10 decimal

    cpu.clock_cycle(&mut mem);

    // -4 = 0xFFFF_FFFC, 0b1010 | 0xFFFF_FFFC = 0xFFFF_FFFE
    assert_eq!(cpu.reg[1], 0xFFFF_FFFE);
}

#[test]
fn ori_with_all_ones_register() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ORI x3, x1, 0b1010
    let ori_instruction: Word = 0b000000001010_00001_110_00011_0010011;
    mem.store_word(0x0, ori_instruction);

    cpu.reg[1] = u32::MAX; // all ones

    cpu.clock_cycle(&mut mem);

    // all ones | 0b1010 = all ones
    assert_eq!(cpu.reg[3], u32::MAX);
}

#[test]
fn ori_with_zero_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ORI x2, x1, 0
    let ori_instruction: Word = 0b000000000000_00001_110_00010_0010011;
    mem.store_word(0x0, ori_instruction);

    cpu.reg[1] = 0b1101;

    cpu.clock_cycle(&mut mem);

    // x1 | 0 = x1
    assert_eq!(cpu.reg[2], 0b1101);
}

#[test]
fn ori_with_all_ones_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // ORI x2, x1, -1 (0xFFF sign-extended = 0xFFFF_FFFF)
    let ori_instruction: Word = 0b111111111111_00001_110_00010_0010011;
    mem.store_word(0x0, ori_instruction);

    cpu.reg[1] = 0b101010;

    cpu.clock_cycle(&mut mem);

    // x1 | -1 = -1 (all ones)
    assert_eq!(cpu.reg[2], u32::MAX);
}

#[test]
fn xori_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: XORI x1, x0, 5
    let xori_instruction: Word = 0b000000000101_00000_100_00001_0010011; // imm=5, rs1=x0, funct3=100, rd=x1, opcode=0010011
    mem.store_word(0x0, xori_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, xori_instruction);
}

#[test]
fn xori_regular_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XORI x3, x2, 0xF0F
    let xori_instruction: Word = 0b111100001111_00010_100_00011_0010011;
    mem.store_word(0x0, xori_instruction);

    cpu.reg[2] = 0xAAAA_AAAA; // rs1 = 0xAAAA_AAAA

    cpu.clock_cycle(&mut mem);

    // Expected: 0xAAAA_AAAA XOR 0xF0F0 = 0xAAAA_5A5A
    assert_eq!(cpu.reg[3], 0xAAAA_AAAA ^ 0xFFFF_FF0F);
}

#[test]
fn xori_with_zero_register() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XORI x1, x0, 0xFF (should just set x1 = 0xFF)
    let xori_instruction: Word = 0b000011111111_00000_100_00001_0010011;
    mem.store_word(0x0, xori_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 0xFF);
}

#[test]
fn xori_negative_immediate() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XORI x1, x2, -1 (imm = 0xFFF in 12-bit two's complement)
    let xori_instruction: Word = 0b111111111111_00010_100_00001_0010011;
    mem.store_word(0x0, xori_instruction);

    cpu.reg[2] = 0x12345678;

    cpu.clock_cycle(&mut mem);

    // Expected: x1 = x2 XOR 0xFFFF_FFFF = bitwise NOT of x2
    assert_eq!(cpu.reg[1], !0x12345678);
}

#[test]
fn xori_all_ones_register() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // XORI x4, x1, 0xF0
    let xori_instruction: Word = 0b000011110000_00001_100_00100_0010011;
    mem.store_word(0x0, xori_instruction);

    cpu.reg[1] = 0xFFFF_FFFF;

    cpu.clock_cycle(&mut mem);

    // Expected: 0xFFFF_FFFF XOR 0x0000_00F0 = 0xFFFF_FF0F
    assert_eq!(cpu.reg[4], 0xFFFF_FF0F);
}

#[test]
fn not_pseudo_instruction() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // NOT x5, x6  ==  XORI x5, x6, -1
    let not_instruction: Word = 0b111111111111_00110_100_00101_0010011;
    mem.store_word(0x0, not_instruction);

    cpu.reg[6] = 0xDEADBEEF;

    cpu.clock_cycle(&mut mem);

    // Expected: x5 = ~x6
    assert_eq!(cpu.reg[5], !0xDEADBEEF);
}

/// Test fetching an SLLI instruction from memory
#[test]
fn slli_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: SLLI x1, x2, 3
    // Format: funct7(0000000), shamt(00011), rs1(00010), funct3(001), rd(00001), opcode(0010011)
    let slli_instruction: Word = 0b0000000_00011_00010_001_00001_0010011;
    mem.store_word(0x0, slli_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, slli_instruction);
}

/// Basic operation: Shift left logical immediate
#[test]
fn slli_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLLI x1, x2, 2  -> x1 = x2 << 2
    let slli_instruction: Word = 0b0000000_00010_00010_001_00001_0010011;
    mem.store_word(0x0, slli_instruction);

    cpu.reg[2] = 5; // 0b0101

    cpu.clock_cycle(&mut mem);

    // 5 << 2 = 20
    assert_eq!(cpu.reg[1], 20);
}

/// Shift left by zero (no change)
#[test]
fn slli_with_zero_shift() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLLI x3, x4, 0
    let slli_instruction: Word = 0b0000000_00000_00100_001_00011_0010011;
    mem.store_word(0x0, slli_instruction);

    cpu.reg[4] = 0x12345678;

    cpu.clock_cycle(&mut mem);

    // Shift by 0 → unchanged
    assert_eq!(cpu.reg[3], 0x12345678);
}

/// Shift with large shift amount (e.g. 31 bits)
#[test]
fn slli_with_large_shift_amount() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLLI x5, x6, 31
    let slli_instruction: Word = 0b0000000_11111_00110_001_00101_0010011;
    mem.store_word(0x0, slli_instruction);

    cpu.reg[6] = 1;

    cpu.clock_cycle(&mut mem);

    // 1 << 31 = 0x80000000
    assert_eq!(cpu.reg[5], 0x80000000);
}

/// Shift left by a small value (check bit movement)
#[test]
fn slli_small_shift_behavior() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLLI x7, x8, 4
    let slli_instruction: Word = 0b0000000_00100_01000_001_00111_0010011;
    mem.store_word(0x0, slli_instruction);

    cpu.reg[8] = 0x0000_00F0; // 240

    cpu.clock_cycle(&mut mem);

    // 0xF0 << 4 = 0xF00
    assert_eq!(cpu.reg[7], 0x0000_0F00);
}

/// Shifting zero (should remain zero)
#[test]
fn slli_with_reg_0() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLLI x1, x0, 5
    let slli_instruction: Word = 0b0000000_00101_00000_001_00001_0010011;
    mem.store_word(0x0, slli_instruction);

    cpu.clock_cycle(&mut mem);

    // 0 << anything = 0
    assert_eq!(cpu.reg[1], 0);
}

/// Shifting with all-ones register value
#[test]
fn slli_with_all_ones_register() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLLI x3, x1, 1
    let slli_instruction: Word = 0b0000000_00001_00001_001_00011_0010011;
    mem.store_word(0x0, slli_instruction);

    cpu.reg[1] = u32::MAX; // 0xFFFF_FFFF

    cpu.clock_cycle(&mut mem);

    // Shift left 1: drops top bit, becomes 0xFFFF_FFFE
    assert_eq!(cpu.reg[3], 0xFFFF_FFFE);
}

/// Shifting with sign bit (ensure logical shift, not arithmetic)
#[test]
fn slli_logical_shift_behavior() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLLI x2, x3, 1
    let slli_instruction: Word = 0b0000000_00001_00011_001_00010_0010011;
    mem.store_word(0x0, slli_instruction);

    // x3 = 0x80000000 (MSB set)
    cpu.reg[3] = 0x80000000;

    cpu.clock_cycle(&mut mem);

    // Logical shift left by 1 = 0x00000000 (MSB bit shifted out)
    assert_eq!(cpu.reg[2], 0x00000000);
}

/// SLLI with a middle register and mid-sized shift
#[test]
fn slli_mid_shift_example() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLLI x10, x11, 8
    let slli_instruction: Word = 0b0000000_01000_01011_001_01010_0010011;
    mem.store_word(0x0, slli_instruction);

    cpu.reg[11] = 0x0000_00AB;

    cpu.clock_cycle(&mut mem);

    // 0xAB << 8 = 0xAB00
    assert_eq!(cpu.reg[10], 0x0000_AB00);
}

/// Test fetching an SRLI instruction from memory
#[test]
fn srli_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: SRLI x1, x2, 3
    // funct7 = 0000000 (SRLI), shamt = 00011, rs1 = 00010, funct3 = 101, rd = 00001, opcode = 0010011
    let srli_instruction: Word = 0b0000000_00011_00010_101_00001_0010011;
    mem.store_word(0x0, srli_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, srli_instruction);
}

/// Basic operation: Shift right logical immediate
#[test]
fn srli_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRLI x1, x2, 2  -> x1 = x2 >> 2 (logical)
    let srli_instruction: Word = 0b0000000_00010_00010_101_00001_0010011;
    mem.store_word(0x0, srli_instruction);

    cpu.reg[2] = 16; // 0b10000

    cpu.clock_cycle(&mut mem);

    // 16 >> 2 = 4
    assert_eq!(cpu.reg[1], 4);
}

/// Shift right by zero (no change)
#[test]
fn srli_with_zero_shift() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRLI x3, x4, 0
    let srli_instruction: Word = 0b0000000_00000_00100_101_00011_0010011;
    mem.store_word(0x0, srli_instruction);

    cpu.reg[4] = 0x12345678;

    cpu.clock_cycle(&mut mem);

    // No shift → unchanged
    assert_eq!(cpu.reg[3], 0x12345678);
}

/// Shift right by maximum amount (31)
#[test]
fn srli_with_large_shift_amount() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRLI x5, x6, 31
    let srli_instruction: Word = 0b0000000_11111_00110_101_00101_0010011;
    mem.store_word(0x0, srli_instruction);

    cpu.reg[6] = 0x80000000;

    cpu.clock_cycle(&mut mem);

    // Logical right shift by 31: only LSB remains (1)
    assert_eq!(cpu.reg[5], 1);
}

/// Shift right logical should NOT sign-extend
#[test]
fn srli_logical_shift_behavior() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRLI x2, x3, 1
    let srli_instruction: Word = 0b0000000_00001_00011_101_00010_0010011;
    mem.store_word(0x0, srli_instruction);

    // x3 = 0x80000000 (MSB set)
    cpu.reg[3] = 0x80000000;

    cpu.clock_cycle(&mut mem);

    // Logical shift right → introduces 0s on the left
    // 0x80000000 >> 1 = 0x40000000
    assert_eq!(cpu.reg[2], 0x40000000);
}

/// Shift with all-ones register
#[test]
fn srli_with_all_ones_register() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRLI x3, x1, 4
    let srli_instruction: Word = 0b0000000_00100_00001_101_00011_0010011;
    mem.store_word(0x0, srli_instruction);

    cpu.reg[1] = u32::MAX; // 0xFFFF_FFFF

    cpu.clock_cycle(&mut mem);

    // 0xFFFF_FFFF >> 4 = 0x0FFF_FFFF
    assert_eq!(cpu.reg[3], 0x0FFF_FFFF);
}

/// Shifting zero (always zero)
#[test]
fn srli_with_reg_0() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRLI x1, x0, 10
    let srli_instruction: Word = 0b0000000_01010_00000_101_00001_0010011;
    mem.store_word(0x0, srli_instruction);

    cpu.clock_cycle(&mut mem);

    // 0 >> anything = 0
    assert_eq!(cpu.reg[1], 0);
}

/// Shift right logical with a random value and small shift
#[test]
fn srli_small_shift_behavior() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRLI x7, x8, 3
    let srli_instruction: Word = 0b0000000_00011_01000_101_00111_0010011;
    mem.store_word(0x0, srli_instruction);

    cpu.reg[8] = 0b10100000; // 0xA0

    cpu.clock_cycle(&mut mem);

    // 0b10100000 >> 3 = 0b00010100 (0x14)
    assert_eq!(cpu.reg[7], 0x14);
}

/// SRLI should zero-fill even if input had sign bit set
#[test]
fn srli_does_not_sign_extend() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRLI x10, x11, 4
    let srli_instruction: Word = 0b0000000_00100_01011_101_01010_0010011;
    mem.store_word(0x0, srli_instruction);

    // x11 = 0xF0000000 (high nibble set)
    cpu.reg[11] = 0xF0000000;

    cpu.clock_cycle(&mut mem);

    // Logical right shift by 4 = 0x0F000000 (no sign extension)
    assert_eq!(cpu.reg[10], 0x0F000000);
}

/// Test fetching an SRAI instruction from memory
#[test]
fn srai_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: SRAI x1, x2, 3
    // funct7 = 0100000 (SRAI), shamt = 00011, rs1 = 00010, funct3 = 101, rd = 00001, opcode = 0010011
    let srai_instruction: Word = 0b0100000_00011_00010_101_00001_0010011;
    mem.store_word(0x0, srai_instruction);

    let instruction: u32 = cpu.fetch_instruction(&mut mem);
    assert_eq!(instruction, srai_instruction);
}

/// Basic operation: Shift right arithmetic immediate
#[test]
fn srai_basic_operation() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRAI x1, x2, 2  -> x1 = x2 >> 2 (arithmetic)
    let srai_instruction: Word = 0b0100000_00010_00010_101_00001_0010011;
    mem.store_word(0x0, srai_instruction);

    cpu.reg[2] = 16; // 0b0001_0000 (positive)

    cpu.clock_cycle(&mut mem);

    // 16 >> 2 = 4
    assert_eq!(cpu.reg[1], 4);
}

/// Arithmetic shift should preserve sign bit (negative numbers)
#[test]
fn srai_preserves_sign_bit() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRAI x3, x4, 1
    let srai_instruction: Word = 0b0100000_00001_00100_101_00011_0010011;
    mem.store_word(0x0, srai_instruction);

    // -8 in 32-bit two's complement = 0xFFFF_FFF8
    cpu.reg[4] = (-8i32) as u32;

    cpu.clock_cycle(&mut mem);

    // Arithmetic right shift by 1: 0xFFFF_FFFC (still negative)
    assert_eq!(cpu.reg[3], 0xFFFF_FFFC);
}

/// Shift right by zero (no change)
#[test]
fn srai_with_zero_shift() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRAI x5, x6, 0
    let srai_instruction: Word = 0b0100000_00000_00110_101_00101_0010011;
    mem.store_word(0x0, srai_instruction);

    cpu.reg[6] = 0x12345678;

    cpu.clock_cycle(&mut mem);

    // Shift by 0 → unchanged
    assert_eq!(cpu.reg[5], 0x12345678);
}

/// Shift by maximum (31 bits)
#[test]
fn srai_with_large_shift_amount() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRAI x7, x8, 31
    let srai_instruction: Word = 0b0100000_11111_01000_101_00111_0010011;
    mem.store_word(0x0, srai_instruction);
    mem.store_word(0x4, srai_instruction);

    // Positive number
    cpu.reg[8] = 0x40000000;
    cpu.clock_cycle(&mut mem);
    assert_eq!(cpu.reg[7], 0x0);

    // Negative number
    cpu.reg[8] = 0x80000000;
    cpu.clock_cycle(&mut mem);
    assert_eq!(cpu.reg[7], 0xFFFFFFFF);
}

/// SRAI should sign-extend properly for small shifts
#[test]
fn srai_sign_extension_small_shift() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRAI x9, x10, 3
    let srai_instruction: Word = 0b0100000_00011_01010_101_01001_0010011;
    mem.store_word(0x0, srai_instruction);

    // -16 (0xFFFFFFF0)
    cpu.reg[10] = (-16i32) as u32;

    cpu.clock_cycle(&mut mem);

    // Arithmetic shift right 3: expected -2 (0xFFFFFFFE)
    assert_eq!(cpu.reg[9], 0xFFFFFFFE);
}

/// Shifting with all ones register
#[test]
fn srai_with_all_ones_register() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRAI x11, x1, 4
    let srai_instruction: Word = 0b0100000_00100_00001_101_01011_0010011;
    mem.store_word(0x0, srai_instruction);

    cpu.reg[1] = u32::MAX; // all ones (-1)

    cpu.clock_cycle(&mut mem);

    // Arithmetic right shift keeps -1 (sign bit stays)
    assert_eq!(cpu.reg[11], u32::MAX);
}

/// Shifting zero should remain zero
#[test]
fn srai_with_reg_0() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRAI x1, x0, 10
    let srai_instruction: Word = 0b0100000_01010_00000_101_00001_0010011;
    mem.store_word(0x0, srai_instruction);

    cpu.clock_cycle(&mut mem);

    assert_eq!(cpu.reg[1], 0);
}

/// Mixed test: positive and negative number comparison
#[test]
fn srai_mixed_sign_behavior() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRAI x12, x13, 4
    let srai_instruction: Word = 0b0100000_00100_01101_101_01100_0010011;
    mem.store_word(0x0, srai_instruction);
    mem.store_word(0x4, srai_instruction);

    // Case 1: Positive
    cpu.reg[13] = 0x0000_F000;
    cpu.clock_cycle(&mut mem);
    assert_eq!(cpu.reg[12], 0x00000F00);

    // Case 2: Negative
    cpu.reg[13] = (-4096i32) as u32; // 0xFFFF_F000
    cpu.clock_cycle(&mut mem);
    assert_eq!(cpu.reg[12], 0xFFFF_FF00);
}

/// SRAI should maintain top bits filled with sign bit when shifting multiple times
#[test]
fn srai_multiple_sign_extension_bits() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SRAI x14, x15, 8
    let srai_instruction: Word = 0b0100000_01000_01111_101_01110_0010011;
    mem.store_word(0x0, srai_instruction);

    // Negative number
    cpu.reg[15] = 0xFFFF_0000;

    cpu.clock_cycle(&mut mem);

    // Right shift by 8 → should stay negative
    // Expected: 0xFFFFFF00
    assert_eq!(cpu.reg[14], 0xFFFFFF00);
}
