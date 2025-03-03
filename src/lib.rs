//#![no_std]
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
    use frand::Rand;

    #[test]
    fn test_1k() {
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

    #[test]
    fn test_statistical() {
        let mut bloom = MicroBloom::<256, 3>::new();
        let mut rand = Rand::with_seed(0);

        for _ in 0..3000u32 {
            bloom.insert(rand.r#gen());
        }
        let mut false_positives: u32 = 0;
        for _ in 0..1000u32 {
            if bloom.check(rand.r#gen()) {
                false_positives += 1;
            }
        }
        let false_positives_percentage: f32 = false_positives as f32 / 1000.0;
        assert!(0.27 < false_positives_percentage && false_positives_percentage < 0.31);

        for _ in 0..13000u32 {
            bloom.insert(rand.r#gen());
        }
        let mut false_positives: u32 = 0;
        for _ in 0..1000u32 {
            if bloom.check(rand.r#gen()) {
                false_positives += 1;
            }
        }
        let false_positives_percentage: f32 = false_positives as f32 / 1000.0;
        assert!(0.97 < false_positives_percentage && false_positives_percentage < 1.00);
    }
}
