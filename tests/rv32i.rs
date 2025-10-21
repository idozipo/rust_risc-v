use rust_risc_v::*;

/* =========================================================
RV32I TEST SUITE
Covers: LUI, AUIPC, JAL, JALR, BRANCH, LOAD, STORE,
        ALU-IMM, ALU-REG, and control flow integration.
========================================================= */

/* -------------------- LUI & AUIPC -------------------- */

#[test]
fn lui_and_auipc_basic() {
    let mut cpu = RISCV::reset();
    let mut mem = Memory::new();

    // LUI x1, 0x12345  => x1 = 0x12345000
    let lui_x1: Word = 0b00010010001101000101_00001_0110111;
    mem.store_word(0x00, lui_x1);

    // AUIPC x2, 0x1    => x2 = PC + (1 << 12) = 0x0004 + 0x1000 = 0x1004
    let auipc_x2: Word = 0b00000000000000000001_00010_0010111;
    mem.store_word(0x04, auipc_x2);

    cpu.clock_cycle(&mut mem); // LUI
    cpu.clock_cycle(&mut mem); // AUIPC

    assert_eq!(cpu.reg[1], 0x12345000);
    assert_eq!(cpu.reg[2], 0x1004);
    assert_eq!(cpu.pc, 0x8);
}

/* -------------------- JAL and JALR -------------------- */

#[test]
fn jal_and_jalr_basic_flow() {
    let mut cpu = RISCV::reset();
    let mut mem = Memory::new();

    // JAL x1, +8 -> jumps to 0x08, x1 = return address (0x4)
    let jal_x1_plus8: Word = 0b0_0000000100_0_00000000_00001_1101111;
    mem.store_word(0x00, jal_x1_plus8);

    // ADDI x2, x0, 99 -> target at 0x08
    let addi_x2_99: Word = 0b000001100011_00000_000_00010_0010011;
    mem.store_word(0x08, addi_x2_99);

    // JALR x0, 0(x1) -> jump back to 0x4 (infinite loop)
    let jalr_x0_x1_0: Word = 0b000000000000_00001_000_00000_1100111;
    mem.store_word(0x0C, jalr_x0_x1_0);

    cpu.clock_cycle(&mut mem); // JAL to 0x08
    assert_eq!(cpu.pc, 0x08);
    assert_eq!(cpu.reg[1], 0x4);

    cpu.clock_cycle(&mut mem); // ADDI x2, x0, 99
    assert_eq!(cpu.reg[2], 99);

    cpu.clock_cycle(&mut mem);
    assert_eq!(cpu.pc, 0x04);
}

/* -------------------- ALU-Immediate -------------------- */

#[test]
fn alu_immediate_ops() {
    let mut cpu = RISCV::reset();
    let mut mem = Memory::new();

    // ADDI x3, x0, 5
    let addi_x3_5: Word = 0b000000000101_00000_000_00011_0010011;
    // SLTI x4, x3, 10  -> x4 = 1
    let slti_x4_x3_10: Word = 0b000000001010_00011_010_00100_0010011;
    // ANDI x5, x3, 3   -> 5 & 3 = 1
    let andi_x5_x3_3: Word = 0b000000000011_00011_111_00101_0010011;
    // ORI  x6, x3, 8   -> 5 | 8 = 13
    let ori_x6_x3_8: Word = 0b000000001000_00011_110_00110_0010011;
    // XORI x7, x3, 6   -> 5 ^ 6 = 3
    let xori_x7_x3_6: Word = 0b000000000110_00011_100_00111_0010011;

    mem.store_word(0x00, addi_x3_5);
    mem.store_word(0x04, slti_x4_x3_10);
    mem.store_word(0x08, andi_x5_x3_3);
    mem.store_word(0x0C, ori_x6_x3_8);
    mem.store_word(0x10, xori_x7_x3_6);

    for _ in 0..5 {
        cpu.clock_cycle(&mut mem);
    }

    assert_eq!(cpu.reg[3], 5);
    assert_eq!(cpu.reg[4], 1);
    assert_eq!(cpu.reg[5], 1);
    assert_eq!(cpu.reg[6], 13);
    assert_eq!(cpu.reg[7], 3);
}

/* -------------------- ALU Register -------------------- */

#[test]
fn alu_register_ops() {
    let mut cpu = RISCV::reset();
    let mut mem = Memory::new();

    cpu.reg[1] = 8;
    cpu.reg[2] = 3;

    // ADD x3, x1, x2 -> 11
    let add_x3 = 0b0000000_00010_00001_000_00011_0110011;
    // SUB x4, x1, x2 -> 5
    let sub_x4 = 0b0100000_00010_00001_000_00100_0110011;
    // AND x5, x1, x2 -> 8 & 3 = 0
    let and_x5 = 0b0000000_00010_00001_111_00101_0110011;
    // OR x6, x1, x2 -> 8 | 3 = 11
    let or_x6 = 0b0000000_00010_00001_110_00110_0110011;
    // XOR x7, x1, x2 -> 8 ^ 3 = 11
    let xor_x7 = 0b0000000_00010_00001_100_00111_0110011;
    // SLL x8, x2, x1 (3 << 8 = 768)
    let sll_x8 = 0b0000000_00001_00010_001_01000_0110011;
    // SRL x9, x1, x2 (8 >> 3 = 1)
    let srl_x9 = 0b0000000_00010_00001_101_01001_0110011;

    let instrs = [add_x3, sub_x4, and_x5, or_x6, xor_x7, sll_x8, srl_x9];
    for (i, &inst) in instrs.iter().enumerate() {
        mem.store_word(i * 4, inst);
    }

    for _ in 0..instrs.len() {
        cpu.clock_cycle(&mut mem);
    }

    assert_eq!(cpu.reg[3], 11);
    assert_eq!(cpu.reg[4], 5);
    assert_eq!(cpu.reg[5], 0);
    assert_eq!(cpu.reg[6], 11);
    assert_eq!(cpu.reg[7], 11);
    assert_eq!(cpu.reg[8], 768);
    assert_eq!(cpu.reg[9], 1);
}

/* -------------------- LOAD + STORE combined -------------------- */

#[test]
fn load_and_store_interaction() {
    let mut cpu = RISCV::reset();
    let mut mem = Memory::new();

    cpu.reg[10] = 0xABCD1234;
    cpu.reg[1] = 0x100;

    // SW x10, 0(x1)
    let sw_instr: Word = 0b0000000_01010_00001_010_00000_0100011;
    // LW x11, 0(x1)
    let lw_instr: Word = 0b000000000000_00001_010_01011_0000011;

    mem.store_word(0x00, sw_instr);
    mem.store_word(0x04, lw_instr);

    cpu.clock_cycle(&mut mem); // SW
    cpu.clock_cycle(&mut mem); // LW

    assert_eq!(mem.fetch_word(0x100), 0xABCD1234);
    assert_eq!(cpu.reg[11], 0xABCD1234);
}

/* -------------------- BRANCH INTEGRATION -------------------- */

#[test]
fn branch_and_loop_program() {
    let mut cpu = RISCV::reset();
    let mut mem = Memory::new();

    // Program: sum 1+2+3+4+5 = 15

    // 0x00: ADDI x1, x0, 0   ; sum = 0
    let addi_sum = 0b000000000000_00000_000_00001_0010011;
    // 0x04: ADDI x2, x0, 1   ; i = 1
    let addi_i = 0b000000000001_00000_000_00010_0010011;
    // 0x08: ADDI x3, x0, 6   ; limit = 6
    let addi_limit = 0b000000000110_00000_000_00011_0010011;
    // 0x0C: ADD  x1, x1, x2  ; sum += i
    let add_sum = 0b0000000_00010_00001_000_00001_0110011;
    // 0x10: ADDI x2, x2, 1   ; i++
    let inc_i = 0b000000000001_00010_000_00010_0010011;
    // 0x14: BLT x2, x3, -8   ; if (i < limit) jump to 0x0C
    let blt_back = 0b1_111111_00011_00010_100_1100_1_1100011;

    let instrs: [u32; 6] = [addi_sum, addi_i, addi_limit, add_sum, inc_i, blt_back];
    for (i, &inst) in instrs.iter().enumerate() {
        mem.store_word(i * 4, inst);
    }

    // Run enough cycles to finish loop
    for _ in 0..18 {
        cpu.clock_cycle(&mut mem);
    }

    assert_eq!(cpu.reg[1], 15);
    assert_eq!(cpu.reg[2], 6);
}

/* -------------------- COMPREHENSIVE MIXED PROGRAM -------------------- */

#[test]
fn full_program_integration() {
    let mut cpu = RISCV::reset();
    let mut mem = Memory::new();

    // Simulate a “mini C” program:
    // int a = 5, b = 7;
    // int *ptr = 0x100;
    // *ptr = a * b;
    // if (*ptr >= 30) *ptr += 10;

    cpu.reg[10] = 5; // a
    cpu.reg[11] = 7; // b
    cpu.reg[12] = 0x100; // ptr

    // MUL not part of RV32I, so simulate a*b via loop or shifts:
    // we’ll just compute via addition here.

    // 0x00: ADDI x13, x0, 0 ; acc = 0
    let addi_acc = 0b000000000000_00000_000_01101_0010011;
    // 0x04: Loop: ADD x13, x13, x10
    let add_acc = 0b0000000_01010_01101_000_01101_0110011;
    // 0x08: ADDI x11, x11, -1 ; b--
    let dec_b = 0b111111111111_01011_000_01011_0010011;
    // 0x0C: BNE x11, x0, -8   ; repeat if b != 0
    let bne_loop = 0b1_111111_00000_01011_001_1100_1_1100011;
    // 0x10: SW x13, 0(x12) ; store result
    let sw_res = 0b0000000_01101_01100_010_00000_0100011;
    // 0x14: LW x14, 0(x12) ; load back
    let lw_res = 0b000000000000_01100_010_01110_0000011;
    // 0x18: SLTI x15, x14, 30 ; less than 30?
    let slti_cmp = 0b000000011110_01110_010_01111_0010011;
    // 0x1C: BNE x15, x0, +8 ; if x14 < 30, skip next
    let beq_skip = 0b0000000_00000_01111_001_0100_0_1100011;
    // 0x20: ADDI x14, x14, 10 ; increment by 10
    let addi_plus10 = 0b000000001010_01110_000_01110_0010011;
    // 0x24: SW x14, 0(x12)
    let sw_final = 0b0000000_01110_01100_010_00000_0100011;

    let instrs = [
        addi_acc,
        add_acc,
        dec_b,
        bne_loop,
        sw_res,
        lw_res,
        slti_cmp,
        beq_skip,
        addi_plus10,
        sw_final,
    ];

    for (i, &inst) in instrs.iter().enumerate() {
        mem.store_word(i * 4, inst);
    }

    for i in 0..28 {
        println!(
            "{} | PC: 0x{:02x} X13: {} X14: {} X15: {}",
            i, cpu.pc, cpu.reg[13], cpu.reg[14], cpu.reg[15]
        );
        cpu.clock_cycle(&mut mem);
    }

    let result = mem.fetch_word(0x100);
    assert_eq!(result, 45); // 5 * 7 = 35 >= 30, so +10
}
