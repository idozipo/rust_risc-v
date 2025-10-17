use std::u32;

use rust_risc_v::*;

#[test]
fn sign_extend_0() {
    let value: usize = 0b0000_0000;
    let bits: u32 = 8;
    let extended: i32 = sign_extend_u32(value, bits);
    assert_eq!(extended, 0);
}

#[test]
fn sign_extend_max_u32() {
    let value: usize = 0b1111_1111;
    let bits: u32 = 8;
    let extended: i32 = sign_extend_u32(value, bits);
    assert_eq!(extended, -1);
}

#[test]
fn sign_extend_32_bits_positive() {
    let value: usize = 0b0111_1111_1111_1111_1111_1111_1111_1111;
    let bits: u32 = 32;
    let extended: i32 = sign_extend_u32(value, bits);
    assert_eq!(extended, 2147483647);
}

#[test]
fn sign_extend_32_bits_negative() {
    let value: usize = 0b1111_1111_1111_1111_1111_1111_1111_1111;
    let bits: u32 = 32;
    let extended: i32 = sign_extend_u32(value, bits);
    assert_eq!(extended, -1);
}

#[test]
#[should_panic(expected = "bits must be less than or equal to 32")]
fn sign_extend_more_than_32_bits() {
    let value: usize = 0b0000_0000_0000_0000_0000_0000_0000_0000;
    let bits: u32 = 40; // More than 32 bits
    let extended: i32 = sign_extend_u32(value, bits);
    assert_eq!(extended, 0); // Should still return 0
}

#[test]
fn opcode_values() {
    assert_eq!(OPCODE::OPIMM.value(), 0b0010011);
    assert_eq!(OPCODE::LUI.value(), 0b0110111);
    assert_eq!(OPCODE::AUIPC.value(), 0b0010111);
    assert_eq!(OPCODE::OPRR.value(), 0b0110011);
    assert_eq!(OPCODE::JAL.value(), 0b1101111);
    assert_eq!(OPCODE::JALR.value(), 0b1100111);
    assert_eq!(OPCODE::BRANCH.value(), 0b1100011);
    assert_eq!(OPCODE::LOAD.value(), 0b0000011);
    assert_eq!(OPCODE::STORE.value(), 0b0100011);
    assert_eq!(OPCODE::FENCE.value(), 0b0001111);
    assert_eq!(OPCODE::SYSTEM.value(), 0b1110011);
}

#[test]
fn opcode_from_value() {
    assert_eq!(OPCODE::from_value(0b0010011), Some(OPCODE::OPIMM));
    assert_eq!(OPCODE::from_value(0b0110111), Some(OPCODE::LUI));
    assert_eq!(OPCODE::from_value(0b0010111), Some(OPCODE::AUIPC));
    assert_eq!(OPCODE::from_value(0b0110011), Some(OPCODE::OPRR));
    assert_eq!(OPCODE::from_value(0b1101111), Some(OPCODE::JAL));
    assert_eq!(OPCODE::from_value(0b1100111), Some(OPCODE::JALR));
    assert_eq!(OPCODE::from_value(0b1100011), Some(OPCODE::BRANCH));
    assert_eq!(OPCODE::from_value(0b0000011), Some(OPCODE::LOAD));
    assert_eq!(OPCODE::from_value(0b0100011), Some(OPCODE::STORE));
    assert_eq!(OPCODE::from_value(0b0001111), Some(OPCODE::FENCE));
    assert_eq!(OPCODE::from_value(0b1110011), Some(OPCODE::SYSTEM));
    assert_eq!(OPCODE::from_value(0b0000000), None); // Invalid opcode
}

#[test]
fn get_opcode_from_instruction() {
    // Example instruction: ADDI x1, x0, 10 (opcode for OPIMM is 0b0010011)
    let instruction: u32 = 0b00000000001000000000000010010011; // This is the binary representation of the instruction
    let opcode: Option<OPCODE> = OPCODE::get_opcode(instruction);
    assert_eq!(opcode, Some(OPCODE::OPIMM));
}

#[test]
fn failure_of_unrecognized_opcode_from_instruction() {
    // Example instruction with an invalid opcode (0b0000000)
    let instruction: u32 = 0;
    let opcode: Option<OPCODE> = OPCODE::get_opcode(instruction);
    assert_eq!(opcode, None);
}

#[test]
fn regular_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mem: Memory = Memory::new();

    let instruction: u32 = cpu.fetch_instruction_word(&mem);
    assert_eq!(instruction, 0); // Memory is empty, so fetching from address 0 should return 0
}

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
