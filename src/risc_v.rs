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

/// OPCODE always occupies the lowest 7 bits of an instruction
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OPCODE {
    OPIMM,  // Integer Register-Immediate Instructions
    LUI,    // Load Upper Immediate
    AUIPC,  // Add upper immediate to PC
    OPRR,   // Integer Register-Register Operations
    JAL,    // Jump and link
    JALR,   // Jump and link register
    BRANCH, // Conditional Branches
    LOAD,   // Loads values from memory
    STORE,  // Stores values to memory
    FENCE,  // Memory and I/O fence
    SYSTEM, // Environment call and breakpoints
}

impl OPCODE {
    pub fn encoding(&self) -> EncodingVariant {
        match self {
            OPCODE::OPIMM => EncodingVariant::IType,
            OPCODE::LUI => EncodingVariant::UType,
            OPCODE::AUIPC => EncodingVariant::UType,
            OPCODE::OPRR => EncodingVariant::RType,
            OPCODE::JAL => EncodingVariant::JType,
            OPCODE::JALR => EncodingVariant::IType,
            OPCODE::BRANCH => EncodingVariant::BType,
            OPCODE::LOAD => EncodingVariant::IType,
            OPCODE::STORE => EncodingVariant::SType,
            // TODO: handle these properly
            OPCODE::FENCE => EncodingVariant::IType, // technically I-type
            OPCODE::SYSTEM => EncodingVariant::IType, // technically I-type
        }
    }

    pub fn value(&self) -> usize {
        match self {
            OPCODE::OPIMM => 0b0010011,
            OPCODE::LUI => 0b0110111,
            OPCODE::AUIPC => 0b0010111,
            OPCODE::OPRR => 0b0110011,
            OPCODE::JAL => 0b1101111,
            OPCODE::JALR => 0b1100111,
            OPCODE::BRANCH => 0b1100011,
            OPCODE::LOAD => 0b0000011,
            OPCODE::STORE => 0b0100011,
            OPCODE::FENCE => 0b0001111,
            OPCODE::SYSTEM => 0b1110011,
        }
    }

    pub fn from_value(value: usize) -> Option<Self> {
        use OPCODE::*;
        match value {
            x if x == OPIMM.value() => Some(OPIMM),
            x if x == LUI.value() => Some(LUI),
            x if x == AUIPC.value() => Some(AUIPC),
            x if x == OPRR.value() => Some(OPRR),
            x if x == JAL.value() => Some(JAL),
            x if x == JALR.value() => Some(JALR),
            x if x == BRANCH.value() => Some(BRANCH),
            x if x == LOAD.value() => Some(LOAD),
            x if x == STORE.value() => Some(STORE),
            x if x == FENCE.value() => Some(FENCE),
            x if x == SYSTEM.value() => Some(SYSTEM),
            _ => None,
        }
    }

    pub fn get_opcode(instruction: Word) -> Option<Self> {
        let opcode_value: usize = (instruction & 0b1111111) as usize; // get the lowest 7 bits
        OPCODE::from_value(opcode_value)
    }
}

pub enum EncodingVariant {
    RType, // register to register operations
    IType, // immediate to register operations
    SType, // store operations
    BType, // branch operations
    UType, // upper immediate operations
    JType, // jump operations
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
    fn fetch_instruction_word(&mut self, mem: &Memory) -> Word {
        let instruction: Word = mem.fetch_word(self.pc as usize);
        self.pc += 4; // increment the pc by a word 

        instruction
    }

    pub fn execute(&mut self, mem: &Memory) -> Word {
        let insruction: Word = self.fetch_instruction_word(mem);
        // TODO: handle unrecognized opcodes properly
        let opcode: OPCODE = OPCODE::get_opcode(insruction).unwrap(); // get the opcode from the instruction

        insruction
    }
}
