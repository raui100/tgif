use std::collections::BTreeMap;

use crate::encode::{rice_index, u8_to_rice_code};

pub fn u8_to_vec_bool(remainder_bits: u8, remainder_range: u8) -> BTreeMap<u8, Vec<bool>> {
    debug_assert_eq!(remainder_range, 2_u8.pow(remainder_bits as u32));
    let mut codebook = BTreeMap::new();
    for num in 0..=u8::MAX {
        let index = rice_index(num);
        let code = u8_to_rice_code(index, remainder_bits, remainder_range);
        codebook.insert(num, code);
    }

    codebook
}

#[test]
fn test_u8_to_vec_bool() {
    use crate::encode::{remainder_coding, unary_coding};
    for remainder_bits in 1..8_u8 {
        let remainder_range = 2_u8.pow(remainder_bits as u32);
        let codebook = u8_to_vec_bool(remainder_bits, remainder_range);
        for ind in 0..=u8::MAX {
            let val = codebook.get(&ind).unwrap();
            let index = rice_index(ind);
            let mut bools = unary_coding(index >> remainder_bits);
            if remainder_bits > 0 {
                let rem = remainder_coding(
                    index & (2u8.pow(remainder_bits as u32) - 1),
                    remainder_bits
                );

                bools.extend( rem);
            }
            assert_eq!(
                *val,
                bools,
                "rem={}, ind={}", remainder_bits, ind
            );
        }
    }
}

// Decoding with BTree is slower than using the numbers
pub fn _vec_bool_to_u8(remainder_bits: u8, remainder_range: u8) -> BTreeMap<Vec<bool>, u8> {
    debug_assert_eq!(remainder_range, 2_u8.pow(remainder_bits as u32));
    debug_assert!(remainder_bits <= 7);
    let code_book = u8_to_vec_bool(remainder_bits, remainder_range);
    let mut rev_code_book = BTreeMap::new();
    for key in code_book.keys() {
        rev_code_book.insert(code_book.get(key).unwrap().clone(), *key);
    }

    rev_code_book
}

#[test]
fn test_vec_bool_to_u8() {
    for remainder_bits in 0..8 {
        let remainder_range = 2_u8.pow(remainder_bits as u32);
        let codebook = u8_to_vec_bool(remainder_bits, remainder_range);
        let rev_codebook = _vec_bool_to_u8(remainder_bits, remainder_range);
        for ind in 0..=u8::MAX {
            let bools = codebook.get(&ind).unwrap();
            assert_eq!(
                &ind,
                rev_codebook.get(bools).unwrap()
            )
        }
    }
}