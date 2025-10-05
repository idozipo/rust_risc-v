use rust_risc_v::risc_v::{Memory, RISCV, Word};

/// Sets up the cpu and memory to be used by tests
fn setup() -> (RISCV, Memory) {
    let cpu: RISCV = RISCV::reset();
    let mem: Memory = Memory::new();

    (cpu, mem)
}

#[test]
fn simple_test() {
    let (mut cpu, mut mem) = setup();
    println!("registers: {:?}", cpu.reg);
    println!("pc: {:?}", cpu.pc);

    // start
    mem[0x0] = 0x40;
    mem[0x1] = 0x40;
    mem[0x2] = 0x40;
    mem[0x3] = 0x40;
    // end

    println!("mem[0x0]: {:?}", mem[0x0]);
    let inst_one: Word = cpu.execute(&mem);
    println!("mem{{0x0}}: {:?}", inst_one);
}
