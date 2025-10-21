use std::{
    ops::{Index, IndexMut},
    usize,
};

use crate::sign_extend_u32;

pub type Byte = u8; // Represents a byte in memory
pub type HalfWord = u16; // Represents 2 bytes in memory
pub type Word = u32; // Represents 4 bytes in memory (one instruction)

const XLEN: usize = 32; // # of registers ( mem_size = 2^(xlen-1) )
const MEM_SIZE: usize = 0x1000000;

pub struct Memory {
    mem: Vec<Byte>, //[Byte; MEM_SIZE], // 16MB of addresable memory
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: vec![0; MEM_SIZE],
        }
    }

    /// Reads a full word from memory at aligned address
    pub fn fetch_word(&self, addr: usize) -> Word {
        assert!(addr % 4 == 0); // the address needs to be aligned to 32 bits
        assert!(
            addr <= MEM_SIZE - 4,
            "Tried accessing memory out of bounds {}",
            addr
        ); // ensure we don't go out of bounds

        // little-endian
        u32::from_le_bytes([self[addr], self[addr + 1], self[addr + 2], self[addr + 3]])
    }

    pub fn store_word(&mut self, addr: usize, value: Word) {
        assert!(addr % 4 == 0); // the address needs to be aligned to 32 bits
        assert!(addr <= MEM_SIZE - 4, "Tried accessing memory out of bounds"); // ensure we don't go out of bounds

        let bytes: [Byte; 4] = value.to_le_bytes(); // convert the word to bytes (little-endian)
        self.mem[addr] = bytes[0];
        self.mem[addr + 1] = bytes[1];
        self.mem[addr + 2] = bytes[2];
        self.mem[addr + 3] = bytes[3];
    }

    /// Reads a half word from memory at aligned address
    pub fn fetch_halfword(&self, addr: usize) -> HalfWord {
        assert!(addr % 2 == 0); // the address needs to be aligned to 16 bits
        assert!(addr <= MEM_SIZE - 2, "Tried accessing memory out of bounds"); // ensure we don't go out of bounds

        // little-endian
        u16::from_le_bytes([self[addr], self[addr + 1]])
    }

    pub fn store_halfword(&mut self, addr: usize, value: HalfWord) {
        assert!(addr % 2 == 0); // the address needs to be aligned to 16 bits
        assert!(addr <= MEM_SIZE - 2, "Tried accessing memory out of bounds"); // ensure we don't go out of bounds

        let bytes: [Byte; 2] = value.to_le_bytes(); // convert the word to bytes (little-endian)
        self.mem[addr] = bytes[0];
        self.mem[addr + 1] = bytes[1];
    }
}

impl Index<usize> for Memory {
    type Output = Byte;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < MEM_SIZE);
        &self.mem[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < MEM_SIZE);
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
    // OPIMM
    ADDI { imm: i32, rs1: usize, rd: usize },
    SLTI { imm: i32, rs1: usize, rd: usize },
    STLIU { imm: u32, rs1: usize, rd: usize },
    ANDI { imm: u32, rs1: usize, rd: usize },
    ORI { imm: u32, rs1: usize, rd: usize },
    XORI { imm: u32, rs1: usize, rd: usize },
    SLLI { shamt: u32, rs1: usize, rd: usize },
    SRLI { shamt: u32, rs1: usize, rd: usize },
    SRAI { shamt: u32, rs1: usize, rd: usize },
    // LUI \ AUIPC
    LUI { imm: u32, rd: usize },
    AUIPC { imm: u32, rd: usize },
    // OPRR
    ADD { rs1: usize, rs2: usize, rd: usize },
    SUB { rs1: usize, rs2: usize, rd: usize },
    SLT { rs1: usize, rs2: usize, rd: usize },
    SLTU { rs1: usize, rs2: usize, rd: usize },
    AND { rs1: usize, rs2: usize, rd: usize },
    OR { rs1: usize, rs2: usize, rd: usize },
    XOR { rs1: usize, rs2: usize, rd: usize },
    SLL { rs1: usize, rs2: usize, rd: usize },
    SRL { rs1: usize, rs2: usize, rd: usize },
    SRA { rs1: usize, rs2: usize, rd: usize },
    // JAL \ JALR
    JAL { offset: i32, rd: usize },
    JALR { offset: i32, rs1: usize, rd: usize },
    // BRANCH
    BEQ { offset: i32, rs1: usize, rs2: usize },
    BNE { offset: i32, rs1: usize, rs2: usize },
    BLT { offset: i32, rs1: usize, rs2: usize },
    BLTU { offset: i32, rs1: usize, rs2: usize },
    BGE { offset: i32, rs1: usize, rs2: usize },
    BGEU { offset: i32, rs1: usize, rs2: usize },
    LW { offset: i32, rs1: usize, rd: usize },
    LH { offset: i32, rs1: usize, rd: usize },
    LHU { offset: i32, rs1: usize, rd: usize },
    LB { offset: i32, rs1: usize, rd: usize },
    LBU { offset: i32, rs1: usize, rd: usize },
    SW { offset: i32, rs1: usize, rs2: usize },
    SH { offset: i32, rs1: usize, rs2: usize },
    SB { offset: i32, rs1: usize, rs2: usize },
    // TODO: implement these as we go along
}

impl Instruction {
    const ADDI_FUNCT3: usize = 0b000;
    const SLTI_FUNCT3: usize = 0b010;
    const SLTIU_FUNCT3: usize = 0b011;
    const ANDI_FUNCT3: usize = 0b111;
    const ORI_FUNCT3: usize = 0b110;
    const XORI_FUNCT3: usize = 0b100;
    const SLLI_FUNCT3: usize = 0b001;
    const SRLI_FUNCT3: usize = 0b101;
    const SRAI_FUNCT3: usize = 0b101;

    const ADD_FUNCT3: usize = 0b000;
    const ADD_FUNCT7: usize = 0b0000000;
    const SUB_FUNCT3: usize = 0b000;
    const SUB_FUNCT7: usize = 0b0100000;
    const SLT_FUNCT3: usize = 0b010;
    const SLT_FUNCT7: usize = 0b0000000;
    const SLTU_FUNCT3: usize = 0b011;
    const SLTU_FUNCT7: usize = 0b0000000;
    const AND_FUNCT3: usize = 0b111;
    const AND_FUNCT7: usize = 0b0000000;
    const OR_FUNCT3: usize = 0b110;
    const OR_FUNCT7: usize = 0b0000000;
    const XOR_FUNCT3: usize = 0b100;
    const XOR_FUNCT7: usize = 0b0000000;
    const SLL_FUNCT3: usize = 0b001;
    const SLL_FUNCT7: usize = 0b0000000;
    const SRL_FUNCT3: usize = 0b101;
    const SRL_FUNCT7: usize = 0b0000000;
    const SRA_FUNCT3: usize = 0b101;
    const SRA_FUNCT7: usize = 0b0100000;

    const JALR_FUNCT3: usize = 0b000;

    const BEQ_FUNCT3: usize = 0b000;
    const BNE_FUNCT3: usize = 0b001;
    const BLT_FUNCT3: usize = 0b100;
    const BLTU_FUNCT3: usize = 0b110;
    const BGE_FUNCT3: usize = 0b101;
    const BGEU_FUNCT3: usize = 0b111;

    const LW_FUNCT3: usize = 0b010;
    const LH_FUNCT3: usize = 0b001;
    const LHU_FUNCT3: usize = 0b101;
    const LB_FUNCT3: usize = 0b000;
    const LBU_FUNCT3: usize = 0b100;

    const SW_FUNCT3: usize = 0b010;
    const SH_FUNCT3: usize = 0b001;
    const SB_FUNCT3: usize = 0b000;

    const OPIMM_BITS: u32 = 12;
    const JAL_BITS: u32 = 21;
    const JALR_BITS: u32 = 12;
    const BRANCH_BITS: u32 = 13;
    const LOAD_BITS: u32 = 12;
    const STORE_BITS: u32 = 12;

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
                    let addi_imm: i32 = sign_extend_u32(imm, Instruction::OPIMM_BITS);
                    Instruction::ADDI {
                        imm: addi_imm,
                        rs1: rs1,
                        rd: rd,
                    }
                } else if opcode == OPCODE::OPIMM && funct3 == Instruction::SLTI_FUNCT3 {
                    let slti_imm: i32 = sign_extend_u32(imm, Instruction::OPIMM_BITS);
                    Instruction::SLTI {
                        imm: slti_imm,
                        rs1: rs1,
                        rd: rd,
                    }
                } else if opcode == OPCODE::OPIMM && funct3 == Instruction::SLTIU_FUNCT3 {
                    let sltiu_imm: u32 = sign_extend_u32(imm, Instruction::OPIMM_BITS) as u32;
                    Instruction::STLIU {
                        imm: sltiu_imm,
                        rs1: rs1,
                        rd: rd,
                    }
                } else if opcode == OPCODE::OPIMM && funct3 == Instruction::ANDI_FUNCT3 {
                    let andi_imm: u32 = sign_extend_u32(imm, Instruction::OPIMM_BITS) as u32;
                    Instruction::ANDI {
                        imm: andi_imm,
                        rs1: rs1,
                        rd: rd,
                    }
                } else if opcode == OPCODE::OPIMM && funct3 == Instruction::ORI_FUNCT3 {
                    let ori_imm: u32 = sign_extend_u32(imm, Instruction::OPIMM_BITS) as u32;
                    Instruction::ORI {
                        imm: ori_imm,
                        rs1: rs1,
                        rd: rd,
                    }
                } else if opcode == OPCODE::OPIMM && funct3 == Instruction::XORI_FUNCT3 {
                    let xori_imm: u32 = sign_extend_u32(imm, Instruction::OPIMM_BITS) as u32;
                    Instruction::XORI {
                        imm: xori_imm,
                        rs1: rs1,
                        rd: rd,
                    }
                } else if opcode == OPCODE::OPIMM && funct3 == Instruction::SLLI_FUNCT3 {
                    let shamt: u32 = (imm & 0b11111) as u32; // shift amount is in lower 5 bits
                    Instruction::SLLI { shamt, rs1, rd }
                } else if opcode == OPCODE::OPIMM
                    && funct3 == Instruction::SRLI_FUNCT3
                    && (imm >> 5) == 0
                {
                    let shamt: u32 = (imm & 0b11111) as u32; // shift amount is in lower 5 bits
                    Instruction::SRLI { shamt, rs1, rd }
                } else if opcode == OPCODE::OPIMM
                    && funct3 == Instruction::SRAI_FUNCT3
                    && (imm >> 5) == 0b0100000
                {
                    let shamt: u32 = (imm & 0b11111) as u32; // shift amount is in lower 5 bits
                    Instruction::SRAI { shamt, rs1, rd }
                } else if opcode == OPCODE::JALR && funct3 == Instruction::JALR_FUNCT3 {
                    let jalr_imm: i32 = sign_extend_u32(imm, Instruction::JALR_BITS);
                    Instruction::JALR {
                        offset: jalr_imm,
                        rs1,
                        rd,
                    }
                } else if opcode == OPCODE::LOAD && funct3 == Instruction::LW_FUNCT3 {
                    let offset: i32 = sign_extend_u32(imm, Instruction::LOAD_BITS);
                    Instruction::LW { offset, rs1, rd }
                } else if opcode == OPCODE::LOAD && funct3 == Instruction::LH_FUNCT3 {
                    let offset: i32 = sign_extend_u32(imm, Instruction::LOAD_BITS);
                    Instruction::LH { offset, rs1, rd }
                } else if opcode == OPCODE::LOAD && funct3 == Instruction::LHU_FUNCT3 {
                    let offset: i32 = sign_extend_u32(imm, Instruction::LOAD_BITS);
                    Instruction::LHU { offset, rs1, rd }
                } else if opcode == OPCODE::LOAD && funct3 == Instruction::LB_FUNCT3 {
                    let offset: i32 = sign_extend_u32(imm, Instruction::LOAD_BITS);
                    Instruction::LB { offset, rs1, rd }
                } else if opcode == OPCODE::LOAD && funct3 == Instruction::LBU_FUNCT3 {
                    let offset: i32 = sign_extend_u32(imm, Instruction::LOAD_BITS);
                    Instruction::LBU { offset, rs1, rd }
                } else {
                    panic!("unrecognized IType instruction")
                }
            }
            EncodingVariant::UType {
                imm_31_12,
                rd,
                opcode,
            } => {
                if opcode == OPCODE::LUI {
                    let imm: u32 = (imm_31_12 as u32) << 12;
                    Instruction::LUI { imm, rd }
                } else if opcode == OPCODE::AUIPC {
                    let imm: u32 = (imm_31_12 as u32) << 12;
                    Instruction::AUIPC { imm, rd }
                } else {
                    panic!("unrecognized UType instruction")
                }
            }
            EncodingVariant::RType {
                funct7,
                rs2,
                rs1,
                funct3,
                rd,
                opcode,
            } => {
                if opcode == OPCODE::OPRR
                    && funct3 == Instruction::ADD_FUNCT3
                    && funct7 == Instruction::ADD_FUNCT7
                {
                    Instruction::ADD { rs1, rs2, rd }
                } else if opcode == OPCODE::OPRR
                    && funct3 == Instruction::SUB_FUNCT3
                    && funct7 == Instruction::SUB_FUNCT7
                {
                    Instruction::SUB { rs1, rs2, rd }
                } else if opcode == OPCODE::OPRR
                    && funct3 == Instruction::SLT_FUNCT3
                    && funct7 == Instruction::SLT_FUNCT7
                {
                    Instruction::SLT { rs1, rs2, rd }
                } else if opcode == OPCODE::OPRR
                    && funct3 == Instruction::SLTU_FUNCT3
                    && funct7 == Instruction::SLTU_FUNCT7
                {
                    Instruction::SLTU { rs1, rs2, rd }
                } else if opcode == OPCODE::OPRR
                    && funct3 == Instruction::AND_FUNCT3
                    && funct7 == Instruction::AND_FUNCT7
                {
                    Instruction::AND { rs1, rs2, rd }
                } else if opcode == OPCODE::OPRR
                    && funct3 == Instruction::OR_FUNCT3
                    && funct7 == Instruction::OR_FUNCT7
                {
                    Instruction::OR { rs1, rs2, rd }
                } else if opcode == OPCODE::OPRR
                    && funct3 == Instruction::XOR_FUNCT3
                    && funct7 == Instruction::XOR_FUNCT7
                {
                    Instruction::XOR { rs1, rs2, rd }
                } else if opcode == OPCODE::OPRR
                    && funct3 == Instruction::SLL_FUNCT3
                    && funct7 == Instruction::SLL_FUNCT7
                {
                    Instruction::SLL { rs1, rs2, rd }
                } else if opcode == OPCODE::OPRR
                    && funct3 == Instruction::SRL_FUNCT3
                    && funct7 == Instruction::SRL_FUNCT7
                {
                    Instruction::SRL { rs1, rs2, rd }
                } else if opcode == OPCODE::OPRR
                    && funct3 == Instruction::SRA_FUNCT3
                    && funct7 == Instruction::SRA_FUNCT7
                {
                    Instruction::SRA { rs1, rs2, rd }
                } else {
                    todo!()
                }
            }
            EncodingVariant::JType {
                imm_20,
                imm_10_1,
                imm_11,
                imm_19_12,
                rd,
                opcode,
            } => {
                if opcode == OPCODE::JAL {
                    let offset: i32 = sign_extend_u32(
                        (imm_20 << 20) | (imm_19_12 << 12) | (imm_11 << 11) | (imm_10_1 << 1),
                        Instruction::JAL_BITS,
                    );
                    Instruction::JAL { offset, rd }
                } else {
                    panic!("unrecognized JType instruction")
                }
            }
            EncodingVariant::BType {
                imm_12,
                imm_10_5,
                rs2,
                rs1,
                funct3,
                imm_4_1,
                imm_11,
                opcode,
            } => {
                let offset: i32 = sign_extend_u32(
                    (imm_12 << 12) | (imm_11 << 11) | (imm_10_5 << 5) | (imm_4_1 << 1),
                    Instruction::BRANCH_BITS,
                );
                if opcode == OPCODE::BRANCH && funct3 == Instruction::BEQ_FUNCT3 {
                    Instruction::BEQ { offset, rs1, rs2 }
                } else if opcode == OPCODE::BRANCH && funct3 == Instruction::BNE_FUNCT3 {
                    Instruction::BNE { offset, rs1, rs2 }
                } else if opcode == OPCODE::BRANCH && funct3 == Instruction::BLT_FUNCT3 {
                    Instruction::BLT { offset, rs1, rs2 }
                } else if opcode == OPCODE::BRANCH && funct3 == Instruction::BLTU_FUNCT3 {
                    Instruction::BLTU { offset, rs1, rs2 }
                } else if opcode == OPCODE::BRANCH && funct3 == Instruction::BGE_FUNCT3 {
                    Instruction::BGE { offset, rs1, rs2 }
                } else if opcode == OPCODE::BRANCH && funct3 == Instruction::BGEU_FUNCT3 {
                    Instruction::BGEU { offset, rs1, rs2 }
                } else {
                    panic!("unrecognized BType instruction")
                }
            }
            EncodingVariant::SType {
                imm_11_5,
                rs2,
                rs1,
                funct3,
                imm_4_0,
                opcode,
            } => {
                let offset: i32 =
                    sign_extend_u32((imm_11_5 << 5) | imm_4_0, Instruction::STORE_BITS);
                if opcode == OPCODE::STORE && funct3 == Instruction::SW_FUNCT3 {
                    Instruction::SW { offset, rs1, rs2 }
                } else if opcode == OPCODE::STORE && funct3 == Instruction::SH_FUNCT3 {
                    Instruction::SH { offset, rs1, rs2 }
                } else if opcode == OPCODE::STORE && funct3 == Instruction::SB_FUNCT3 {
                    Instruction::SB { offset, rs1, rs2 }
                } else {
                    panic!("unrecognized SType instruction")
                }
            } // _ => todo!("only  instructions are implemented so far"),
        }
    }
}

pub struct RISCV {
    pub reg: [Word; XLEN],     // 32 registers which are 32 bits wide
    pub pc: Word,              // Program counter (holds current instruction address)
    current_instruction: Word, // holds the current instruction being executed
}

impl RISCV {
    pub fn reset() -> Self {
        RISCV {
            reg: [0; XLEN], // Resets registers to 0x00000
            pc: 0,          // Start executing code from 0x00000
            current_instruction: 0,
        }
    }

    pub fn new_() -> Self {
        RISCV {
            reg: [0; XLEN], // Resets registers to 0x00000
            pc: 0x1000,     // Start executing code from 0x1000
            current_instruction: 0,
        }
    }

    pub fn clock_cycle(&mut self, mem: &mut Memory) {
        self.fetch_instruction(mem);
        self.execute(mem);
        self.increment_pc();
    }

    /// Fetch the full instruction word that pc is pointing to (incrementing it)
    pub fn fetch_instruction(&mut self, mem: &Memory) -> Word {
        self.current_instruction = mem.fetch_word(self.pc as usize);

        self.current_instruction
    }

    pub fn increment_pc(&mut self) {
        self.pc = self.pc.wrapping_add(4); // increment pc by 4 (size of instruction word)
    }

    pub fn execute(&mut self, mem: &mut Memory) {
        let encoding: EncodingVariant = EncodingVariant::get_encoding(self.current_instruction);
        let parsed_instruction: Instruction = Instruction::parse_instruction(encoding);
        match parsed_instruction {
            Instruction::ADDI { imm, rs1, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.reg[rs1].wrapping_add(imm as u32);
                }
            }
            Instruction::SLTI { imm, rs1, rd } => {
                if rd != 0 {
                    let rs1_value: i32 = self.reg[rs1] as i32;
                    self.reg[rd] = if rs1_value < imm { 1 } else { 0 };
                }
            }
            Instruction::STLIU { imm, rs1, rd } => {
                if rd != 0 {
                    let rs1_value: u32 = self.reg[rs1];
                    self.reg[rd] = if rs1_value < imm { 1 } else { 0 }
                }
            }
            Instruction::ANDI { imm, rs1, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.reg[rs1] & imm;
                }
            }
            Instruction::ORI { imm, rs1, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.reg[rs1] | imm;
                }
            }
            Instruction::XORI { imm, rs1, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.reg[rs1] ^ imm;
                }
            }
            Instruction::SLLI { shamt, rs1, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.reg[rs1] << shamt;
                }
            }
            Instruction::SRLI { shamt, rs1, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.reg[rs1] >> shamt;
                }
            }
            Instruction::SRAI { shamt, rs1, rd } => {
                if rd != 0 {
                    let rs1_value: i32 = self.reg[rs1] as i32;
                    self.reg[rd] = (rs1_value >> shamt) as u32;
                }
            }
            Instruction::LUI { imm, rd } => {
                if rd != 0 {
                    // x0 is hardwired to 0
                    self.reg[rd] = imm;
                }
            }
            Instruction::AUIPC { imm, rd } => {
                if rd != 0 {
                    // x0 is hardwired to 0
                    self.reg[rd] = self.pc.wrapping_add(imm);
                }
            }
            Instruction::ADD { rs1, rs2, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.reg[rs1].wrapping_add(self.reg[rs2]);
                }
            }
            Instruction::SUB { rs1, rs2, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.reg[rs1].wrapping_sub(self.reg[rs2]);
                }
            }
            Instruction::SLT { rs1, rs2, rd } => {
                if rd != 0 {
                    let rs1_value: i32 = self.reg[rs1] as i32;
                    let rs2_value: i32 = self.reg[rs2] as i32;
                    self.reg[rd] = if rs1_value < rs2_value { 1 } else { 0 };
                }
            }
            Instruction::SLTU { rs1, rs2, rd } => {
                if rd != 0 {
                    let rs1_value: u32 = self.reg[rs1];
                    let rs2_value: u32 = self.reg[rs2];
                    self.reg[rd] = if rs1_value < rs2_value { 1 } else { 0 }
                }
            }
            Instruction::AND { rs1, rs2, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.reg[rs1] & self.reg[rs2];
                }
            }
            Instruction::OR { rs1, rs2, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.reg[rs1] | self.reg[rs2];
                }
            }
            Instruction::XOR { rs1, rs2, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.reg[rs1] ^ self.reg[rs2];
                }
            }
            Instruction::SLL { rs1, rs2, rd } => {
                if rd != 0 {
                    let shamt: u32 = (self.reg[rs2] & 0b11111) as u32; // shift amount is in lower 5 bits
                    let rs1_value: u32 = self.reg[rs1];
                    self.reg[rd] = (rs1_value << shamt) as u32;
                }
            }
            Instruction::SRL { rs1, rs2, rd } => {
                if rd != 0 {
                    let shamt: u32 = (self.reg[rs2] & 0b11111) as u32; // shift amount is in lower 5 bits
                    let rs1_value: u32 = self.reg[rs1];
                    self.reg[rd] = (rs1_value >> shamt) as u32;
                }
            }
            Instruction::SRA { rs1, rs2, rd } => {
                if rd != 0 {
                    let shamt: u32 = (self.reg[rs2] & 0b11111) as u32; // shift amount is in lower 5 bits
                    let rs1_value: i32 = self.reg[rs1] as i32;
                    self.reg[rd] = (rs1_value >> shamt) as u32;
                }
            }
            Instruction::JAL { offset, rd } => {
                assert!(offset % 2 == 0, "JAL offset must be aligned to 2 bytes");
                if rd != 0 {
                    self.reg[rd] = self.pc.wrapping_add(4);
                }
                let target_address: u32 = self.pc.wrapping_add_signed(offset);
                assert!(
                    target_address % 4 == 0,
                    "JAL target address must be aligned to 4 bytes"
                );
                self.pc = target_address.wrapping_sub(4); // subtract 4 because pc will be incremented after execute
            }
            Instruction::JALR { offset, rs1, rd } => {
                if rd != 0 {
                    self.reg[rd] = self.pc.wrapping_add(4);
                }
                let target_address: u32 = self.reg[rs1].wrapping_add_signed(offset) & !1; // set LSB to 0
                assert!(
                    target_address % 4 == 0,
                    "JALR target address must be aligned to 4 bytes"
                );
                self.pc = target_address.wrapping_sub(4); // subtract 4 because pc will be incremented after execute
            }
            Instruction::BEQ { offset, rs1, rs2 } => {
                let target_address: u32 = self.pc.wrapping_add_signed(offset);

                if self.reg[rs1] == self.reg[rs2] {
                    assert!(
                        target_address % 4 == 0,
                        "BEQ target address must be aligned to 4 bytes"
                    );
                    self.pc = target_address.wrapping_sub(4); // subtract 4 because pc will be incremented after execute
                }
            }
            Instruction::BNE { offset, rs1, rs2 } => {
                let target_address: u32 = self.pc.wrapping_add_signed(offset);

                if self.reg[rs1] != self.reg[rs2] {
                    assert!(
                        target_address % 4 == 0,
                        "BNE target address must be aligned to 4 bytes"
                    );
                    self.pc = target_address.wrapping_sub(4); // subtract 4 because pc will be incremented after execute
                }
            }
            Instruction::BLT { offset, rs1, rs2 } => {
                let target_address: u32 = self.pc.wrapping_add_signed(offset);

                if (self.reg[rs1] as i32) < (self.reg[rs2] as i32) {
                    assert!(
                        target_address % 4 == 0,
                        "BLT target address must be aligned to 4 bytes"
                    );
                    self.pc = target_address.wrapping_sub(4); // subtract 4 because pc will be incremented after execute
                }
            }
            Instruction::BLTU { offset, rs1, rs2 } => {
                let target_address: u32 = self.pc.wrapping_add_signed(offset);

                if self.reg[rs1] < self.reg[rs2] {
                    assert!(
                        target_address % 4 == 0,
                        "BLTU target address must be aligned to 4 bytes"
                    );
                    self.pc = target_address.wrapping_sub(4); // subtract 4 because pc will be incremented after execute
                }
            }
            Instruction::BGE { offset, rs1, rs2 } => {
                let target_address: u32 = self.pc.wrapping_add_signed(offset);

                if (self.reg[rs1] as i32) >= (self.reg[rs2] as i32) {
                    assert!(
                        target_address % 4 == 0,
                        "BGE target address must be aligned to 4 bytes"
                    );
                    self.pc = target_address.wrapping_sub(4); // subtract 4 because pc will be incremented after execute
                }
            }
            Instruction::BGEU { offset, rs1, rs2 } => {
                let target_address: u32 = self.pc.wrapping_add_signed(offset);

                if self.reg[rs1] >= self.reg[rs2] {
                    assert!(
                        target_address % 4 == 0,
                        "BGEU target address must be aligned to 4 bytes"
                    );
                    self.pc = target_address.wrapping_sub(4); // subtract 4 because pc will be incremented after execute
                }
            }
            Instruction::LW { offset, rs1, rd } => {
                let effective_address: Word = self.reg[rs1].wrapping_add_signed(offset);
                let loaded_word: Word = mem.fetch_word(effective_address as usize);
                if rd != 0 {
                    self.reg[rd] = loaded_word;
                }
            }
            Instruction::LH { offset, rs1, rd } => {
                let effective_address: Word = self.reg[rs1].wrapping_add_signed(offset);
                let loaded_halfword: HalfWord = mem.fetch_halfword(effective_address as usize);
                if rd != 0 {
                    self.reg[rd] = sign_extend_u32(loaded_halfword as usize, 16) as u32;
                }
            }
            Instruction::LHU { offset, rs1, rd } => {
                let effective_address: Word = self.reg[rs1].wrapping_add_signed(offset);
                let loaded_halfword: HalfWord = mem.fetch_halfword(effective_address as usize);
                if rd != 0 {
                    self.reg[rd] = loaded_halfword as u32;
                }
            }
            Instruction::LB { offset, rs1, rd } => {
                let effective_address: Word = self.reg[rs1].wrapping_add_signed(offset);
                let loaded_byte: Byte = mem[effective_address as usize];
                if rd != 0 {
                    self.reg[rd] = sign_extend_u32(loaded_byte as usize, 8) as u32;
                }
            }
            Instruction::LBU { offset, rs1, rd } => {
                let effective_address: Word = self.reg[rs1].wrapping_add_signed(offset);
                let loaded_byte: Byte = mem[effective_address as usize];
                if rd != 0 {
                    self.reg[rd] = loaded_byte as u32;
                }
            }
            Instruction::SW { offset, rs1, rs2 } => {
                let target_address: Word = self.reg[rs1].wrapping_add_signed(offset);
                let value: Word = self.reg[rs2];
                mem.store_word(target_address as usize, value);
            }
            Instruction::SH { offset, rs1, rs2 } => {
                let target_address: Word = self.reg[rs1].wrapping_add_signed(offset);
                let value: HalfWord = self.reg[rs2] as HalfWord;
                mem.store_halfword(target_address as usize, value);
            }
            Instruction::SB { offset, rs1, rs2 } => {
                let target_address: Word = self.reg[rs1].wrapping_add_signed(offset);
                let value: Byte = self.reg[rs2] as Byte;
                mem[target_address as usize] = value;
            }
        };
    }
}
