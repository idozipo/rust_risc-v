use rust_risc_v::*;

#[test]
fn addi_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example instruction: ADDI x1, x0, 10 (opcode for OPIMM is 0b0010011)
    let addi_instruction: Word = 0b00000000001000000000000010010011; // This is the binary representation of the instruction
    mem.store_word(0x0, addi_instruction); // Store the instruction at address 0

    let instruction: u32 = cpu.fetch_instruction_word(&mem);
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
    cpu.execute(&mem);

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
    cpu.execute(&mem);

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
    cpu.execute(&mem);

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
    cpu.execute(&mem);

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
    cpu.execute(&mem);

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

    let instruction: u32 = cpu.fetch_instruction_word(&mem);
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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);
    assert_eq!(cpu.reg[3], 0);

    // Try another case: x4 = -2 (signed)
    cpu.reg[4] = (-2i32) as u32;
    cpu.execute(&mem);
    assert_eq!(cpu.reg[3], 1);
}

#[test]
fn slti_with_reg_0() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // SLTI x1, x0, 5 → compare 0 < 5 → true → x1 = 1
    let slti_instruction: Word = 0b000000000101_00000_010_00001_0010011;
    mem.store_word(0x0, slti_instruction);

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    let instruction: u32 = cpu.fetch_instruction_word(&mem);
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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

    assert_eq!(cpu.reg[5], 1);
}

#[test]
fn andi_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Example: ANDI x1, x0, 0b1010 (rd=x1, rs1=x0, imm=10)
    let andi_instruction: Word = 0b000000001010_00000_111_00001_0010011;
    mem.store_word(0x0, andi_instruction);

    let instruction: u32 = cpu.fetch_instruction_word(&mem);
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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

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

    cpu.execute(&mem);

    // x1 & -1 = x1
    assert_eq!(cpu.reg[2], 0b101010);
}
