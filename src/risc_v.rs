use std::ops::{Index, IndexMut};

pub type Byte = u8; // Represents a byte in memory
pub type HalfWord = u16; // Represents 2 bytes in memory
pub type Word = u32; // Represents 4 bytes in memory (one instruction)

const XLEN: usize = 32; // # of registers ( mem_size = 2^(xlen-1) )
const MEM_SIZE: usize = 0x10000;

pub struct Memory {
    mem: [Byte; MEM_SIZE], // 64KB of addresable memory
}

impl Memory {
    pub fn new() -> Self {
        Memory { mem: [0; MEM_SIZE] }
    }

    /// Reads a full word from memory at aligned address
    pub fn fetch_word(&self, addr: usize) -> Word {
        assert!(addr % 4 == 0); // the address needs to be aligned to 32 bits

        // little-endian
        u32::from_le_bytes([self[addr], self[addr + 1], self[addr + 2], self[addr + 3]])
    }
}

impl Index<usize> for Memory {
    type Output = Byte;

    fn index(&self, index: usize) -> &Self::Output {
        &self.mem[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mem[index]
    }
}

enum OPCODE {
    OPIMM, // Integer Register-Immediate Instructions
    LUI,   // Load Upper Immediate
    AUIPC, // Add upper immediate to PC
    OPRR,  // Integer Register-Register Operations
    JAL,   // Jump and link
    JALR,  // Jump and link register
}

impl OPCODE {
    fn value(&self) -> usize {
        match self {
            OPCODE::OPIMM => 0x13,
            OPCODE::LUI => 0x37,
            OPCODE::AUIPC => 0x17,
            OPCODE::OPRR => 0x33,
            OPCODE::JAL => 0x6f,
            OPCODE::JALR => 0x67,
        }
    }
}

pub struct RISCV {
    pub reg: [Word; XLEN], // 32 registers which are 32 bits wide
    pub pc: Word,          // Program counter (holds current instruction address)
}

impl RISCV {
    pub fn reset() -> Self {
        RISCV {
            reg: [0; XLEN], // Resets registers to 0x00000
            pc: 0,          // Start executing code from 0x00000
        }
    }

    /// Fetch the full instruction word that pc is pointing to (incrementing it)
    fn fetch_instruction(&mut self, mem: &Memory) -> Word {
        let instruction: Word = mem.fetch_word(self.pc as usize);
        self.pc += 4; // increment the pc by a word 

        instruction
    }

    pub fn execute(&mut self, mem: &Memory) -> Word {
        let insruction: Word = self.fetch_instruction(mem);

        insruction
    }
}
