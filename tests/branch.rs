use rust_risc_v::*;

/// BEQ instruction fetch
#[test]
fn beq_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // BEQ x1, x2, +8  (funct3 = 000, opcode = 1100011)
    let beq_instruction: Word = 0b0_000000_00010_00001_000_0100_0_1100011;
    mem.store_word(0x0, beq_instruction);

    let instruction = cpu.fetch_instruction(&mem);
    assert_eq!(instruction, beq_instruction);
}

/// BEQ taken: basic forward branch
#[test]
fn beq_basic_taken_forward() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // BEQ x1, x2, +8 ; set x1 == x2 so branch taken
    let beq_instr: Word = 0b0_000000_00010_00001_000_0100_0_1100011; // imm = +8
    mem.store_word(0x0, beq_instr);

    cpu.reg[1] = 5;
    cpu.reg[2] = 5;

    cpu.clock_cycle(&mem);

    // branch taken -> PC = 0 + 8
    assert_eq!(cpu.pc, 8);
}

/// BEQ not taken: PC increments by 4
#[test]
fn beq_not_taken_pc_increments() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // BEQ x1, x2, +8 ; x1 != x2 so not taken
    let beq_instr: Word = 0b0_000000_00010_00001_000_0100_0_1100011; // imm = +8
    mem.store_word(0x0, beq_instr);

    cpu.reg[1] = 1;
    cpu.reg[2] = 2;

    cpu.clock_cycle(&mem);

    // not taken -> PC = 0 + 4
    assert_eq!(cpu.pc, 4);
}

/// BEQ backward branch (negative offset)
#[test]
fn beq_backward_jump() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Place BEQ at address 0x10 with imm = -8 -> target = 0x08
    cpu.pc = 0x10;

    // Encoded imm = -8
    let beq_instr: Word = 0b1_111111_00010_00001_000_1100_1_1100011;
    mem.store_word(0x10, beq_instr);

    cpu.reg[1] = 7;
    cpu.reg[2] = 7; // equal -> branch taken to 0x08

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 0x08);
}

/// BEQ with x0 and x0: always equal -> taken
#[test]
fn beq_x0_x0_always_taken() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // BEQ x0, x0, +8  -> comparing zero with zero, branch taken to 0x04
    let beq_instr: Word = 0b0_000000_00000_00000_000_0100_0_1100011;
    mem.store_word(0x0, beq_instr);

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 8);
}

/// BEQ large forward offset (+1024)
#[test]
fn beq_large_forward_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // BEQ x1, x1, +1024
    let beq_instr: Word = 0b0_100000_00001_00001_000_0000_0_1100011;
    mem.store_word(0x0, beq_instr);

    cpu.reg[1] = 0x42; // equal with itself, taken

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 0x400); // jumped to +1024
}

/// BEQ large backward offset (-512)
#[test]
fn beq_large_backward_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // place PC at 0x800, branch -512 -> target 0x600
    cpu.pc = 0x800;
    let beq_instr: Word = 0b1_110000_00001_00001_000_0000_1_1100011;
    mem.store_word(0x800, beq_instr);

    cpu.reg[1] = 0x7;

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 0x600);
}

/// BEQ skipping instructions (interaction with LUI)
#[test]
fn beq_with_lui_interaction() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Layout:
    // 0x00: LUI x2, 0x1
    // 0x04: BEQ x3, x4, +8  -> if equal, skip 0x08 and go to 0x0C
    // 0x08: LUI x5, 0x2   (should be skipped if taken)
    // 0x0C: LUI x6, 0x3

    let lui_x2 = 0b00000000000000000001_00010_0110111;
    mem.store_word(0x00, lui_x2);

    // BEQ x3, x4, +8 ; encoded below
    let beq_x3_x4_plus8: Word = 0b0_000000_00100_00011_000_0100_0_1100011; // rs1=3 rs2=4 imm=+8
    mem.store_word(0x04, beq_x3_x4_plus8);

    let lui_x5 = 0b00000000000000000010_00101_0110111;
    mem.store_word(0x08, lui_x5);

    let lui_x6 = 0b00000000000000000011_00110_0110111;
    mem.store_word(0x0C, lui_x6);

    // Make branch taken by setting x3 == x4
    cpu.reg[3] = 9;
    cpu.reg[4] = 9;

    cpu.clock_cycle(&mem); // LUI x2
    cpu.clock_cycle(&mem); // BEQ (taken -> jump to 0x0C)
    cpu.clock_cycle(&mem); // LUI x6

    assert_eq!(cpu.reg[2], 0x1000);
    assert_eq!(cpu.reg[5], 0); // skipped
    assert_eq!(cpu.reg[6], 0x3000); // executed
    assert_eq!(cpu.pc, 0x10);
}

/// BEQ with AUIPC interaction
#[test]
fn beq_with_auipc_interaction() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // 0x00: AUIPC x5, 1   -> x5 = PC + (1 << 12) = 0x1000
    // 0x04: BEQ   x6, x7, +8 -> if equal jump to 0x0C
    // 0x08: LUI x8, 2     -> should be skipped if BEQ taken
    // 0x0C: LUI x9, 3

    let auipc_x5 = 0b00000000000000000001_00101_0010111;
    mem.store_word(0x00, auipc_x5);

    // BEQ x6, x7, +8 ; rs1=6 rs2=7 imm=+8
    let beq_x6_x7_plus8: Word = 0b0_000000_00111_00110_000_0100_0_1100011;
    mem.store_word(0x04, beq_x6_x7_plus8);

    let lui_x8 = 0b00000000000000000010_01000_0110111;
    mem.store_word(0x08, lui_x8);

    let lui_x9 = 0b00000000000000000011_01001_0110111;
    mem.store_word(0x0C, lui_x9);

    // Make branch taken
    cpu.reg[6] = 0x12;
    cpu.reg[7] = 0x12;

    cpu.clock_cycle(&mem); // AUIPC
    cpu.clock_cycle(&mem); // BEQ taken
    cpu.clock_cycle(&mem); // LUI x9

    assert_eq!(cpu.reg[5], 0x1000);
    assert_eq!(cpu.reg[8], 0); // skipped
    assert_eq!(cpu.reg[9], 0x3000);
    assert_eq!(cpu.pc, 0x10);
}

/// BEQ with an illegal unaligned target should panic (target = PC + 2)
#[test]
#[should_panic]
fn beq_unaligned_target_panics() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // BEQ x1, x1, +2 -> unaligned target (0x2), emulator should panic/raise
    let beq_unaligned: Word = 0b0_000000_00001_00001_000_0001_0_1100011; // imm = +2
    mem.store_word(0x0, beq_unaligned);

    cpu.reg[1] = 1; // equal -> branch taken to 0x2 (unaligned for 32-bit non-compressed)
    cpu.clock_cycle(&mem);
}

/* -------------------- BNE tests -------------------- */

/// BNE instruction fetch
#[test]
fn bne_instruction_fetch() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // BNE x3, x4, +8 (funct3 = 001)
    let bne_instruction: Word = 0b0_000000_00100_00011_001_0100_0_1100011;
    mem.store_word(0x0, bne_instruction);

    let instruction = cpu.fetch_instruction(&mem);
    assert_eq!(instruction, bne_instruction);
}

/// BNE taken: basic forward (x3 != x4)
#[test]
fn bne_basic_taken_forward() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    let bne_instr: Word = 0b0_000000_00100_00011_001_0100_0_1100011; // imm = +8
    mem.store_word(0x0, bne_instr);

    cpu.reg[3] = 1;
    cpu.reg[4] = 2; // not equal -> taken

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 8);
}

/// BNE not taken: registers equal -> PC += 4
#[test]
fn bne_not_taken_pc_increments() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    let bne_instr: Word = 0b0_000000_00100_00011_001_0100_0_1100011; // imm = +8
    mem.store_word(0x0, bne_instr);

    cpu.reg[3] = 5;
    cpu.reg[4] = 5; // equal -> not taken

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 4);
}

/// BNE backward branch (negative offset)
#[test]
fn bne_backward_jump() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Place BNE at 0x10 with imm = -8 -> target = 0x08
    cpu.pc = 0x10;

    let bne_instr: Word = 0b1_111111_00100_00011_001_1100_1_1100011; // imm = -8, funct3=001
    mem.store_word(0x10, bne_instr);

    cpu.reg[3] = 1;
    cpu.reg[4] = 2; // not equal -> branch taken backward

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 0x08);
}

/// BNE with x0: branch when register non-zero vs zero (x5 vs x0)
#[test]
fn bne_x0_nonzero_taken() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // BNE x5, x0, +8 -> if x5 != 0 branch taken
    let bne_instr: Word = 0b0_000000_00000_00101_001_0100_0_1100011; // rs1 = 5, rs2 = 0
    mem.store_word(0x0, bne_instr);

    cpu.reg[5] = 7; // non-zero -> not equal to x0 -> taken

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 8);
}

/// BNE large forward offset (+1024)
#[test]
fn bne_large_forward_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // BNE x3, x3, +1024 -> x3 == x3 so not taken (since BNE), PC+=4
    let bne_instr: Word = 0b0_100000_00011_00011_001_0000_0_1100011;
    mem.store_word(0x0, bne_instr);

    cpu.reg[3] = 0x1;

    cpu.clock_cycle(&mem);

    // BNE with rs1==rs2 not taken -> PC=4
    assert_eq!(cpu.pc, 4);
}

/// BNE large backward offset (-512)
#[test]
fn bne_large_backward_offset() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    cpu.pc = 0x800;

    // BNE x3, x3, -512
    let bne_instr: Word = 0b1_110000_00011_00000_001_0000_1_1100011;
    mem.store_word(0x800, bne_instr);

    cpu.reg[3] = 0x42;

    cpu.clock_cycle(&mem);

    assert_eq!(cpu.pc, 0x600);
}

/// BNE interacting with LUI (branch to skip an instruction)
#[test]
fn bne_with_lui_interaction() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Layout:
    // 0x00: LUI x2, 0x1
    // 0x04: BNE x3, x4, +8  -> if not equal jump to 0x0C
    // 0x08: LUI x5, 0x2 (skipped if branch taken)
    // 0x0C: LUI x6, 0x3

    let lui_x2 = 0b00000000000000000001_00010_0110111;
    mem.store_word(0x00, lui_x2);

    let bne_x3_x4_plus8: Word = 0b0_000000_00100_00011_001_0100_0_1100011; // rs1=3 rs2=4 imm=+8, funct3=001
    mem.store_word(0x04, bne_x3_x4_plus8);

    let lui_x5 = 0b00000000000000000010_00101_0110111;
    mem.store_word(0x08, lui_x5);

    let lui_x6 = 0b00000000000000000011_00110_0110111;
    mem.store_word(0x0C, lui_x6);

    cpu.reg[3] = 1;
    cpu.reg[4] = 2; // not equal -> branch taken

    cpu.clock_cycle(&mem); // LUI x2
    cpu.clock_cycle(&mem); // BNE -> taken (to 0x0C)
    cpu.clock_cycle(&mem); // LUI x6

    assert_eq!(cpu.reg[2], 0x1000);
    assert_eq!(cpu.reg[5], 0); // skipped
    assert_eq!(cpu.reg[6], 0x3000); // executed
    assert_eq!(cpu.pc, 0x10);
}

/// BNE unaligned target -> should panic
#[test]
#[should_panic]
fn bne_unaligned_target_panics() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // BNE x3, x3, +2 -> target = 0x2 (unaligned), simulator should panic/raise
    let bne_unaligned: Word = 0b0_000000_00000_00011_001_0001_0_1100011; // imm = +2
    mem.store_word(0x0, bne_unaligned);

    cpu.reg[3] = 1;
    cpu.clock_cycle(&mem);
}

/// Combined test: BEQ and BNE sequence altering control flow
#[test]
fn beq_bne_control_flow_sequence() {
    let mut cpu: RISCV = RISCV::reset();
    let mut mem: Memory = Memory::new();

    // Layout:
    // 0x00: ADDI x10, x0, 1
    // 0x04: BEQ  x10, x0, +8   -> not taken (1 != 0)
    // 0x08: BNE  x10, x0, +8   -> taken (1 != 0), jump to 0x10
    // 0x0C: ADDI x11, x0, 0    -> should be skipped
    // 0x10: ADDI x11, x0, 5

    let addi_x10_1: Word = 0b000000000001_00000_000_01010_0010011; // addi x10,x0,1
    let beq_x10_x0_plus8: Word = 0b0_000000_00000_01010_000_0100_0_1100011; // BEQ x10,x0,+8  (rs1=10 rs2=0)
    let bne_x10_x0_plus8: Word = 0b0_000000_00000_01010_001_0100_0_1100011; // BNE x10,x0,+8
    let addi_x11_0: Word = 0b000000000000_00000_000_01011_0010011; // addi x11,x0,0
    let addi_x11_5: Word = 0b000000000101_00000_000_01011_0010011; // addi x11,x0,5

    mem.store_word(0x00, addi_x10_1);
    mem.store_word(0x04, beq_x10_x0_plus8);
    mem.store_word(0x08, bne_x10_x0_plus8);
    mem.store_word(0x0C, addi_x11_0);
    mem.store_word(0x10, addi_x11_5);

    cpu.clock_cycle(&mem); // addi x10 = 1
    cpu.clock_cycle(&mem); // beq -> not taken (x10 != x0) -> PC = 0x08
    cpu.clock_cycle(&mem); // bne -> taken -> PC = 0x10
    cpu.clock_cycle(&mem); // addi x11 = 5

    assert_eq!(cpu.reg[10], 1);
    assert_eq!(cpu.reg[11], 5);
    assert_eq!(cpu.pc, 0x14); // after executing addi at 0x10 -> PC points to 0x14
}
