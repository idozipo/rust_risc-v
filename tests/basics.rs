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

// TODO: test each function in cpu seperately
