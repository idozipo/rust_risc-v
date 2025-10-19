use std::fs;

use crate::Word;

/// Sign-extend a value with the given number of bits to a 32-bit signed integer.
pub fn sign_extend_u32(value: usize, bits: u32) -> i32 {
    assert!(bits <= 32, "bits must be less than or equal to 32");

    let shift = 32 - bits; // number of bits to shift left/right
    ((value << shift) as i32) >> shift
}

pub fn load_from_file() -> Vec<Word> {
    let contents = fs::read("program.bin");
    match contents {
        Ok(bytes) => {
            if bytes.len() % 4 != 0 {
                panic!("File size is not a multiple of 4 bytes");
            }

            let mut words: Vec<Word> = Vec::new();
            let mut i = 0;
            while i + 4 <= bytes.len() {
                let word = (bytes[i] as Word)
                    | ((bytes[i + 1] as Word) << 8)
                    | ((bytes[i + 2] as Word) << 16)
                    | ((bytes[i + 3] as Word) << 24);
                words.push(word);
                i += 4;
            }
            words
        }
        Err(err) => {
            panic!("Failed to read file: {}", err);
        }
    }
}
