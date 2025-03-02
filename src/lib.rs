#![no_std]
use xxhash_rust::xxh32::xxh32;

pub struct MicroBloom<const M: usize, const K: u8> {
    body: [u32; M],
}

impl<const M: usize, const K: u8> MicroBloom<M, K> {
    pub fn new() -> Self {
        MicroBloom { body: [0u32; M] }
    }

    fn get_coordinates(value: u32, seed: u32) -> (usize, u32) {
        let bytes: [u8; 4] = value.to_le_bytes();
        let bit_num: u32 = xxh32(&bytes, seed) % ((M * 32) as u32);
        ((bit_num / 32) as usize, bit_num % 32)
    }

    pub fn insert(&mut self, x: u32) {
        for i in 0..(K as u32) {
            let coordinates = Self::get_coordinates(x, i);
            self.body[coordinates.0] |= 1 << coordinates.1;
        }
    }

    pub fn check(&self, x: u32) -> bool {
        for i in 0..(K as u32) {
            let coordinates = Self::get_coordinates(x, i);

            if self.body[coordinates.0] & (1 << coordinates.1) == 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_insert_1k() {
        let mut bloom = MicroBloom::<256, 3>::new();

        for i in 0..1000u32 {
            bloom.insert(i);
        }
        for i in 0..1000u32 {
            assert!(bloom.check(i));
        }

        let mut false_positive_found = false;
        let mut negative_found = false;
        for i in 1001u32..1000000u32 {
            if !bloom.check(i) {
                negative_found = true;
            } else {
                false_positive_found = true;
            }
            if negative_found && false_positive_found {
                break;
            }
        }
        assert!(false_positive_found);
        assert!(negative_found);
    }
}
