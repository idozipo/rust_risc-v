use rust_risc_v::*;

/// Sets up the cpu and memory to be used by tests
// fn setup_cpu() -> (RISCV, Memory) {
//     let cpu: RISCV = RISCV::reset();
//     let mem: Memory = Memory::new();

//     (cpu, mem)
// }

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
