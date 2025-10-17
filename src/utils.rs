/// Sign-extend a value with the given number of bits to a 32-bit signed integer.
pub fn sign_extend_u32(value: usize, bits: u32) -> i32 {
    assert!(bits <= 32, "bits must be less than or equal to 32");

    let shift = 32 - bits; // number of bits to shift left/right
    ((value << shift) as i32) >> shift
}

#[macro_export]
macro_rules! test_itype_instruction {
    (
        $name:ident,
        instruction = $instr:expr,
        rd = $rd:expr,
        rs1 = $rs1:expr,
        rs1_val = $rs1_val:expr,
        imm = $imm:expr,
        expected = $expected:expr
    ) => {
        #[test]
        fn $name() {
            let mut cpu: RISCV = RISCV::reset();
            let mut mem: Memory = Memory::new();

            mem.store_word(0x0, $instr);

            cpu.reg[$rs1] = $rs1_val;

            cpu.execute(&mem);

            assert_eq!(cpu.reg[$rd], $expected);
        }
    };
}

/// Converts simple I-type assembly into a 32-bit instruction word
/// Usage: asm_itype!(ADDI, rd, rs1, imm)
#[macro_export]
macro_rules! asm_itype {
    ($instr:ident, $rd:expr, $rs1:expr, $imm:expr) => {{
        let opcode = 0b0010011u32;
        let funct3 = match stringify!($instr) {
            "ADDI" => 0b000,
            "SLTI" => 0b010,
            "SLTIU" => 0b011,
            "ANDI" => 0b111,
            "ORI" => 0b110,
            "XORI" => 0b100,
            _ => panic!("Unsupported I-type instruction: {}", stringify!($instr)),
        };
        let imm12 = ($imm as i32 & 0xFFF) as u32; // 12-bit signed immediate
        (imm12 << 20) | (($rs1 as u32) << 15) | (funct3 << 12) | (($rd as u32) << 7) | opcode
    }};
}
