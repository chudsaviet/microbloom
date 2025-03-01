#![no_std]

use xxhash_rust::xxh32::xxh32;

pub enum MicroBloomError {
    Unspecified,
    Size,
}

pub struct MicroBloom<const M: usize, const K: u8> {
    body: [u32; M],
}

impl<const M: usize, const K: u8> MicroBloom<M, K> {
    pub fn new() -> Result<Self, MicroBloomError> {
        Ok(MicroBloom { body: [0u32; M] })
    }

    pub fn insert(&mut self, x: u32) {
        let bytes: [u8; 4] = x.to_le_bytes();
        for i in 0..K {
            let bit_num: u32 = xxh32(&bytes, i as u32);
            let array_position: usize = (bit_num / 32) as usize;
            let bit_position: u32 = bit_num % 32;

            self.body[array_position] |= 1 << bit_position;
        }
    }

    pub fn check(&self, x: u32) -> bool {
        let bytes: [u8; 4] = x.to_le_bytes();
        for i in 0..K {
            let bit_num: u32 = xxh32(&bytes, i as u32);
            let array_position: usize = (bit_num / 32) as usize;
            let bit_position: u32 = bit_num % 32;

            if self.body[array_position] & (1 << bit_position) != 1 {
                return false;
            }
        }

        true
    }
}
