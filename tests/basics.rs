use rust_risc_v::OPCODE;

/// Sets up the cpu and memory to be used by tests
// fn setup_cpu() -> (RISCV, Memory) {
//     let cpu: RISCV = RISCV::reset();
//     let mem: Memory = Memory::new();

//     (cpu, mem)
// }

#[test]
fn test_opcode_values() {
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
fn test_opcode_from_value() {
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
