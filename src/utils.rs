pub fn sign_extend_u32(value: usize, bits: u32) -> i32 {
    assert!(bits <= 32, "bits must be less than or equal to 32");

    let shift = 32 - bits; // number of bits to shift left/right
    ((value << shift) as i32) >> shift
}
