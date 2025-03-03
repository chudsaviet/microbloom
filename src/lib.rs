#![no_std]
use xxhash_rust::xxh32::xxh32;

pub struct MicroBloom<const M: usize, const K: u8> {
    body: [u32; M],
}

impl<const M: usize, const K: u8> MicroBloom<M, K> {
    pub fn new() -> Self {
        MicroBloom { body: [0u32; M] }
    }

    fn get_coordinates(x: &[u8], seed: u32) -> (usize, u32) {
        let bit_num: u32 = xxh32(x, seed) % ((M * 32) as u32);
        ((bit_num / 32) as usize, bit_num % 32)
    }

    pub fn insert(&mut self, x: &[u8]) {
        for i in 0..(K as u32) {
            let coordinates = Self::get_coordinates(x, i);
            self.body[coordinates.0] |= 1 << coordinates.1;
        }
    }

    pub fn check(&self, x: &[u8]) -> bool {
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
            bloom.insert(&i.to_le_bytes());
        }
        for i in 0..1000u32 {
            assert!(bloom.check(&i.to_le_bytes()));
        }

        let mut false_positive_found = false;
        let mut negative_found = false;
        for i in 1001u32..1000000u32 {
            if !bloom.check(&i.to_le_bytes()) {
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

    fn gen_rand_nonce(rand: &mut Rand) -> [u8; 12] {
        let mut result: [u8; 12] = [0u8; 12];
        for i in 0..3 {
            let rnum = rand.r#gen::<u32>().to_le_bytes();
            result[i * 4] = rnum[0];
            result[i * 4 + 1] = rnum[1];
            result[i * 4 + 2] = rnum[2];
            result[i * 4 + 3] = rnum[3];
        }
        result
    }
    // Test statistical characteristics of the Bloom filter.
    // https://hur.st/bloomfilter/?n=3000&p=&m=8192&k=3 was used for theoretical estimations.
    #[test]
    fn test_statistical() {
        let mut bloom = MicroBloom::<256, 3>::new();
        let mut rand = Rand::with_seed(0);

        for _ in 0..3000u32 {
            bloom.insert(&gen_rand_nonce(&mut rand));
        }
        let mut false_positives: u32 = 0;
        for _ in 0..1000u32 {
            if bloom.check(&gen_rand_nonce(&mut rand)) {
                false_positives += 1;
            }
        }
        let false_positives_percentage: f32 = false_positives as f32 / 1000.0;
        assert!(0.27 < false_positives_percentage && false_positives_percentage < 0.31);

        for _ in 0..13000u32 {
            bloom.insert(&gen_rand_nonce(&mut rand));
        }
        let mut false_positives: u32 = 0;
        for _ in 0..1000u32 {
            if bloom.check(&gen_rand_nonce(&mut rand)) {
                false_positives += 1;
            }
        }
        let false_positives_percentage: f32 = false_positives as f32 / 1000.0;
        assert!(0.97 < false_positives_percentage && false_positives_percentage < 1.00);
    }
}
