use rust_risc_v::{Memory, RISCV, Word, load_from_file};

fn main() {
    println!("Hello, world!");
    let instructions: Vec<Word> = load_from_file();
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    let mut i: usize = 0;
    while i < instructions.len() {
        println!("addr 0x{:0X}: 0b{:032b}", 4 * i, instructions[i]);
        let current_instr: Word = instructions[i];

        mem.store_word(4 * i, current_instr);

        i += 1;
    }

    let mut i: usize = 0;
    while i < instructions.len() {
        cpu.clock_cycle(&mut mem);

        i += 1;
    }
}
