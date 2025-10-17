use std::{
    ops::{Index, IndexMut},
    usize,
};

use crate::sign_extend_u32;

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

    pub fn store_word(&mut self, addr: usize, value: Word) {
        assert!(addr % 4 == 0); // the address needs to be aligned to 32 bits

        let bytes: [Byte; 4] = value.to_le_bytes(); // convert the word to bytes (little-endian)
        self.mem[addr] = bytes[0];
        self.mem[addr + 1] = bytes[1];
        self.mem[addr + 2] = bytes[2];
        self.mem[addr + 3] = bytes[3];
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
    // pub fn encoding(&self) -> EncodingVariant {
    //     match self {
    //         OPCODE::OPIMM => EncodingVariant::IType,
    //         OPCODE::LUI => EncodingVariant::UType,
    //         OPCODE::AUIPC => EncodingVariant::UType,
    //         OPCODE::OPRR => EncodingVariant::RType,
    //         OPCODE::JAL => EncodingVariant::JType,
    //         OPCODE::JALR => EncodingVariant::IType,
    //         OPCODE::BRANCH => EncodingVariant::BType,
    //         OPCODE::LOAD => EncodingVariant::IType,
    //         OPCODE::STORE => EncodingVariant::SType,
    //         // TODO: handle these properly
    //         OPCODE::FENCE => EncodingVariant::IType, // technically I-type
    //         OPCODE::SYSTEM => EncodingVariant::IType, // technically I-type
    //     }
    // }

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

#[derive(Debug, PartialEq, Eq)]
pub enum EncodingVariant {
    RType {
        funct7: usize,  // funct variant
        rs2: usize,     // second source register
        rs1: usize,     // first source register
        funct3: usize,  // funct variant
        rd: usize,      // destination register
        opcode: OPCODE, // opcode
    }, // register to register operations
    IType {
        imm: usize,     // immediate value
        rs1: usize,     // source register
        funct3: usize,  // funct variant
        rd: usize,      // destination register
        opcode: OPCODE, // opcode
    }, // immediate to register operations
    SType {
        imm_11_5: usize, // immediate value bits [11:5]
        rs2: usize,      // second source register
        rs1: usize,      // first source register
        funct3: usize,   // funct variant
        imm_4_0: usize,  // immediate value bits [4:0]
        opcode: OPCODE,  // opcode
    }, // store operations
    BType {
        imm_12: usize,   // immediate value bit [12]
        imm_10_5: usize, // immediate value bits [10:5]
        rs2: usize,      // second source register
        rs1: usize,      // first source register
        funct3: usize,   // funct variant
        imm_4_1: usize,  // immediate value bits [4:1]
        imm_11: usize,   // immediate value bit [11]
        opcode: OPCODE,  // opcode
    }, // branch operations
    UType {
        imm_31_12: usize, // immediate value bits [31:12]
        rd: usize,        // destination register
        opcode: OPCODE,   // opcode
    }, // upper immediate operations
    JType {
        imm_20: usize,    // immediate value bit [20]
        imm_10_1: usize,  // immediate value bits [10:1]
        imm_11: usize,    // immediate value bit [11]
        imm_19_12: usize, // immediate value bits [19:12]
        rd: usize,        // destination register
        opcode: OPCODE,   // opcode
    }, // jump operations
}

impl EncodingVariant {
    pub fn get_encoding(instruction: Word) -> EncodingVariant {
        let opcode: OPCODE = OPCODE::get_opcode(instruction).expect(&format!(
            "unrecognized opcode in instruction {:?}",
            instruction
        ));

        match opcode {
            OPCODE::OPIMM => EncodingVariant::IType {
                imm: ((instruction >> 20) & 0b111111111111) as usize,
                rs1: ((instruction >> 15) & 0b11111) as usize,
                funct3: ((instruction >> 12) & 0b111) as usize,
                rd: ((instruction >> 7) & 0b11111) as usize,
                opcode,
            },
            OPCODE::LUI => EncodingVariant::UType {
                imm_31_12: ((instruction >> 12) & 0b11111111111111111111) as usize,
                rd: ((instruction >> 7) & 0b11111) as usize,
                opcode,
            },
            OPCODE::AUIPC => EncodingVariant::UType {
                imm_31_12: ((instruction >> 12) & 0b11111111111111111111) as usize,
                rd: ((instruction >> 7) & 0b11111) as usize,
                opcode,
            },
            OPCODE::OPRR => EncodingVariant::RType {
                funct7: ((instruction >> 25) & 0b1111111) as usize,
                rs2: ((instruction >> 20) & 0b11111) as usize,
                rs1: ((instruction >> 15) & 0b11111) as usize,
                funct3: ((instruction >> 12) & 0b111) as usize,
                rd: ((instruction >> 7) & 0b11111) as usize,
                opcode,
            },
            OPCODE::JAL => EncodingVariant::JType {
                imm_20: ((instruction >> 31) & 0b1) as usize,
                imm_10_1: ((instruction >> 21) & 0b1111111111) as usize,
                imm_11: ((instruction >> 20) & 0b1) as usize,
                imm_19_12: ((instruction >> 12) & 0b11111111) as usize,
                rd: ((instruction >> 7) & 0b11111) as usize,
                opcode,
            },
            OPCODE::JALR => EncodingVariant::IType {
                imm: ((instruction >> 20) & 0b111111111111) as usize,
                rs1: ((instruction >> 15) & 0b11111) as usize,
                funct3: ((instruction >> 12) & 0b111) as usize,
                rd: ((instruction >> 7) & 0b11111) as usize,
                opcode,
            },
            OPCODE::BRANCH => EncodingVariant::BType {
                imm_12: ((instruction >> 31) & 0b1) as usize,
                imm_10_5: ((instruction >> 25) & 0b111111) as usize,
                rs2: ((instruction >> 20) & 0b11111) as usize,
                rs1: ((instruction >> 15) & 0b11111) as usize,
                funct3: ((instruction >> 12) & 0b111) as usize,
                imm_4_1: ((instruction >> 8) & 0b1111) as usize,
                imm_11: ((instruction >> 7) & 0b1) as usize,
                opcode,
            },
            OPCODE::LOAD => EncodingVariant::IType {
                imm: ((instruction >> 20) & 0b111111111111) as usize,
                rs1: ((instruction >> 15) & 0b11111) as usize,
                funct3: ((instruction >> 12) & 0b111) as usize,
                rd: ((instruction >> 7) & 0b11111) as usize,
                opcode,
            },
            OPCODE::STORE => EncodingVariant::SType {
                imm_11_5: ((instruction >> 25) & 0b1111111) as usize,
                rs2: ((instruction >> 20) & 0b11111) as usize,
                rs1: ((instruction >> 15) & 0b11111) as usize,
                funct3: ((instruction >> 12) & 0b111) as usize,
                imm_4_0: ((instruction >> 7) & 0b11111) as usize,
                opcode,
            },
            OPCODE::FENCE => todo!(),
            OPCODE::SYSTEM => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    ADDI { imm: i32, rs1: usize, rd: usize },
    // TODO: implement these as we go along
}

impl Instruction {
    const ADDI_FUNCT3: usize = 0b000;

    const ADDI_IMM_BITS: u32 = 12;

    pub fn parse_instruction(encoding: EncodingVariant) -> Instruction {
        match encoding {
            EncodingVariant::IType {
                imm,
                rs1,
                funct3,
                rd,
                opcode,
            } => {
                if opcode == OPCODE::OPIMM && funct3 == Instruction::ADDI_FUNCT3 {
                    let addi_imm: i32 = sign_extend_u32(imm, Instruction::ADDI_IMM_BITS);
                    Instruction::ADDI {
                        imm: addi_imm,
                        rs1: rs1,
                        rd: rd,
                    }
                } else {
                    todo!()
                }
            }
            _ => todo!(),
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
    pub fn fetch_instruction_word(&mut self, mem: &Memory) -> Word {
        let instruction: Word = mem.fetch_word(self.pc as usize);
        self.pc += 4; // increment the pc by a word 

        instruction
    }

    pub fn get_funct3() {}

    pub fn execute(&mut self, mem: &Memory) {
        let instruction: Word = self.fetch_instruction_word(mem); // fetch the instruction at pc
        let encoding: EncodingVariant = EncodingVariant::get_encoding(instruction);
        let parsed_instruction: Instruction = Instruction::parse_instruction(encoding);
        match parsed_instruction {
            Instruction::ADDI { imm, rs1, rd } => {
                self.reg[rd] = self.reg[rs1].wrapping_add(imm as u32);
            }
        };
    }
}
