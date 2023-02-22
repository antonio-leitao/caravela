use std::collections::HashMap;

struct MaskedHasher {
    mask: u16,
}

fn apply_mask_fast(value: u16, mask: u16) -> u8 {
    let mut result: u8 = 0;
    let mut bit_index: u8 = 0;

    let mut m = mask;
    while m != 0 {
        let bit = m.trailing_zeros() as u8;
        m &= m - 1; // unset the lowest set bit
        result |= ((value >> bit) & 1) << bit_index;
        bit_index += 1;
    }
    result
}



impl MaskedHasher {
    fn new() -> Self {
        // randomly generate a mask with exactly 8 set bits
        let mut mask: u16 = 0;
        let mut bits_set = 0;
        while bits_set < 8 {
            let bit = 1u16 << rand::random::<u8>() % 16;
            if (mask & bit) == 0 {
                mask |= bit;
                bits_set += 1;
            }
        }
        MaskedHasher { mask }
    }
}

impl std::hash::Hasher for MaskedHasher {
    fn write_u16(&mut self, value: u16) {
        let result = apply_mask_fast(value, self.mask);
        self.write_u8(result);
    }

    fn finish(&self) -> u64 {
        0
    }
}

