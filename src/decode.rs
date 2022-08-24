use log::debug;

use crate::args::Args;

enum Coding {
    Unary,
    Remainder,
}

struct State {
    coding: Coding,
    unary: u8,
    remainder: u8,
    remainder_index: u8,
}

impl State {
    fn reset() -> State {
        State {
            coding: Coding::Unary,
            unary: 0,
            remainder: 0,
            remainder_index: 0,
        }
    }
}

pub fn decode(args: &Args) -> Vec<u8> {
    debug!("Reading the TGIF file from disk");
    let tgif = std::fs::read(&args.src)
        .unwrap_or_else(|_| panic!("Failed reading {}", &args.src));


    // The FPGA doesn't produce a header and has hardcoded values
    let (_name, img_width, img_height, parallel_units, remainder_bits, start_index) =
        match &args.no_header {
            true => { ("TGIF no_header", 8, 4, 4, 2, 0) }
            false => {
                let _name = std::str::from_utf8(&tgif[0..4])
                    .expect("Failed reading format name from header. Try the '--no-header' flag");

                let img_width = tgif[4..8].iter()
                    .fold(0_u32, |res, val| (res << 8) + (*val as u32));

                let img_height = tgif[8..12].iter()
                    .fold(0_u32, |res, val| (res << 8) + (*val as u32));

                let parallel_units = tgif[12..16].iter()
                    .fold(0_u32, |res, val| (res << 8) + (*val as u32));

                let remainder_bits = tgif[16];

                (_name, img_width, img_height, parallel_units, remainder_bits, 17)
            }
        };
    assert!(tgif.len() > start_index as usize);
    let img_code_bool: Vec<bool> = tgif[start_index..]
        .iter()
        .flat_map(u8_to_array_bool)
        .collect();


    let unordered_deltas = decode_rice(
        &img_code_bool,
        (img_height * img_width) as usize,
        remainder_bits,
    );
    assert_eq!(unordered_deltas.len(), (img_height * img_width) as usize);

    todo!("Decode!")
}

fn u8_to_array_bool(num: &u8) -> [bool; 8] {
    [
        num & 128 > 0,
        num & 64 > 0,
        num & 32 > 0,
        num & 16 > 0,
        num & 8 > 0,
        num & 4 > 0,
        num & 2 > 0,
        num & 1 > 0,
    ]
}

#[test]
fn test_u8_to_array_bool() {
    assert_eq!(
        u8_to_array_bool(&0),
        [false, false, false, false, false, false, false, false, ]
    );

    assert_eq!(
        u8_to_array_bool(&255),
        [true, true, true, true, true, true, true, true, ]
    );

    assert_eq!(
        u8_to_array_bool(&1),
        [false, false, false, false, false, false, false, true]
    );

    assert_eq!(
        u8_to_array_bool(&254),
        [true, true, true, true, true, true, true, false]
    );

    assert_eq!(
        u8_to_array_bool(&128),
        [true, false, false, false, false, false, false, false]
    );
}

fn decode_rice(code: &[bool], number_of_pixel: usize, remainder_bits: u8) -> Vec<u8> {
    let mut img_code_u8: Vec<u8> = Vec::with_capacity(number_of_pixel);
    let mut state = State::reset();
    let mut pixels = 0;
    for bool in code.iter() {
        // The state decides if the bit(=bool) is part of the unary coding of the remainder coding
        match state.coding {
            Coding::Unary => {  // The current bit (=bool) is part of the unary coding
                match bool {
                    true => { state.unary += 1 }  // Unary coding continues
                    false => { state.coding = Coding::Remainder }  // End of unary coding
                }
            }
            Coding::Remainder => {  // The current bit (=bool) is part of the unary coding

                // The remainder has a size of `remainder_bits` (eg: 2)
                // We can model this for '11' as (((0 << 1) + 1) << 1) + 1)
                state.remainder = (state.remainder << 1) + (*bool as u8);

                // We have to keep track of the size of the remainder (in bits)
                state.remainder_index += 1;
                if state.remainder_index == remainder_bits {
                    // Adding the unary coding and the remainder and pushing this to the vec
                    img_code_u8.push((state.unary << remainder_bits) + state.remainder);
                    pixels += 1;

                    // Due to the bit padding at the end of the file we have to break out of the
                    // loop manually, which is `number_of_pixel - 1` due to zero-indexing
                    if pixels == number_of_pixel {
                        return img_code_u8;

                    } else {
                        state = State::reset();

                    }
                }
            }
        }
    }

    panic!("We should always return early via return function!")
}

#[test]
fn test_decode_rice() {
    let code_0 = vec![false, false, false];  // == 0
    let code_3 = vec![false, true, true];  // == 3
    let code_7 = vec![true, false, true, true]; // == 7
    let code_11 = vec![true, true, false, true, true]; // == 11

    // Testing single numbers
    assert_eq!(decode_rice(&code_0, 1, 2), vec![0]);
    assert_eq!(decode_rice(&code_3, 1, 2), vec![3]);
    assert_eq!(decode_rice(&code_7, 1, 2), vec![7]);
    assert_eq!(decode_rice(&code_11, 1, 2), vec![11]);

    // Testing "real" vectors
    let code: Vec<bool> = [code_0, code_3, code_7, code_11]
        .iter()
        .flat_map(|v| v.clone())
        .collect();

    assert_eq!(decode_rice(&code, 4, 2, ),
               vec![0, 3, 7, 11]
    )
}

