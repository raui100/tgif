use log::{debug, info};

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

pub fn decode(args: &Args) {
    debug!("Reading the TGIF file from disk");
    let tgif = std::fs::read(&args.src).unwrap_or_else(|_| panic!("Failed reading {}", &args.src));

    debug!("Parsing the header");
    let (_name, img_width, img_height, parallel_encoding_units, remainder_bits, start_index) =
        match &args.no_header {  // The metadata is provided via CLI or via file header
            true => (  // The metadata of the TGIF image have been provided via CLI
                       "TGIF",
                       args.width.expect("Check for `None` beforehand!"),
                       args.height.expect("Check for `None` beforehand!"),
                       args.parallel_encoding_units.expect("Check for `None` beforehand!"),
                       args.remainder_bits.expect("Check for `None` beforehand!"),
                       0,  // When there is no header the first byte is already data
            ),

            false => {  // Actually parsing the header
                let name = std::str::from_utf8(&tgif[0..4])
                    .expect("Failed reading format name from header. Try the '--no-header' flag");

                let img_width = tgif[4..8]
                    .iter()
                    .fold(0_u32, |res, val| (res << 8) + (*val as u32));

                let img_height = tgif[8..12]
                    .iter()
                    .fold(0_u32, |res, val| (res << 8) + (*val as u32));

                let parallel_encoding_units = tgif[12..16]
                    .iter()
                    .fold(0_u32, |res, val| (res << 8) + (*val as u32));

                let remainder_bits = tgif[16];

                let start_index = 17;  // The first 17 byte are metadata

                (name, img_width, img_height, parallel_encoding_units, remainder_bits, start_index)
            }
        };

    debug!("Transforming Vec<u8> to Vec<bool>");
    let img_code_bool: Vec<bool> = tgif[start_index..]
        .iter()
        .flat_map(u8_to_array_bool)
        .collect();

    debug!("Decoding Rice-code to Rice-index");
    let unordered_rice_indices = decode_rice(
        &img_code_bool,
        (img_height * img_width) as usize,
        remainder_bits,
    );

    debug!("Reordering the Rice-index");
    let ordered_rice_indices = reorder_img(
        unordered_rice_indices,
        parallel_encoding_units as usize,
        img_width as usize,
    );

    debug!("Transforming the Rice-Index to deltas");
    let deltas = reverse_rice_index(ordered_rice_indices);

    debug!("Transforming the delta back to the original image");
    let img_u8 = reverse_delta(deltas, img_width as usize);

    debug!("Saving the original image to disk");
    image::save_buffer(
        &args.dst,
        &img_u8,
        img_width,
        img_height,
        image::ColorType::L8,
    )
        .unwrap();

    info!("Finished!")
}

/// Reverse index
fn reverse_rice_index(vec: Vec<u8>) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(vec.len());
    for num in vec {
        if (num % 2) == 0 {
            out.push(num / 2);
        } else {
            // if num=255 there is an edge-case where (num + 1) would lead to an overflow
            out.push(0_u8.wrapping_sub(((num as u16 + 1) / 2) as u8));
        }
    }
    out
}

#[test]
fn test_reverse_rice_index() {
    let original = vec![0, 1, 255, 127, 128];
    let rice_index = vec![0, 2, 1, 254, 255];
    assert_eq!(reverse_rice_index(rice_index), original);
}

/// Reverses the delta calculation
fn reverse_delta(delta: Vec<u8>, img_width: usize) -> Vec<u8> {
    let mut img: Vec<u8> = Vec::with_capacity(delta.len());
    let mut prev_num = 0_u8;
    for (ind, delta) in delta.into_iter().enumerate() {
        if ind % img_width == 0 {
            prev_num = 0;
        }
        let num = prev_num.wrapping_sub(delta);
        img.push(num);
        prev_num = num;
    }

    img
}

#[test]
fn test_reverse_delta() {
    // Most simple case
    assert_eq!(reverse_delta(vec![0], 1), vec![0]);

    // Simple case with wrapping sub
    assert_eq!(reverse_delta(vec![1], 1), vec![255]);

    // Two wrapping subs
    assert_eq!(reverse_delta(vec![255, 255], 2), vec![1, 2]);

    // Not wrapping, wrapping, not wrapping, wrapping
    assert_eq!(reverse_delta(vec![0, 1, 254, 3], 4), vec![0, 255, 1, 254]);

    // Delta with reset
    assert_eq!(
        reverse_delta(vec![1, 0, 3, 254], 2),
        vec![255, 255, 253, 255]
    );

    // Another delta with reset
    let original = vec![0, 0, 255, 255, 0, 0, 0, 255];
    let delta = vec![0, 0, 1, 0, 0, 0, 0, 1];
    assert_eq!(reverse_delta(delta, 2), original)
}

/// Reorders a vec that had been built with `parallel_encoding_units`
fn reorder_img(img: Vec<u8>, parallel_encoding_units: usize, img_width: usize) -> Vec<u8> {
    // If only one parallel encoding unit had been used or the image width is one then no
    // reordering is necessary
    if parallel_encoding_units == 1 || img_width == 1 { return img; }

    let mut ordered_vec: Vec<u8> = Vec::with_capacity(img.len());
    let row_len = parallel_encoding_units * img_width;
    for ordered_index in 0..img.len() {
        // Determines in which chunk of rows the index is.
        // Is always a multiple of `img_width` (eg: [0, 1024, 2048, ...] for `img_width=1024`).
        let chunk = (ordered_index / row_len) * row_len;

        // Determines the position within a chunk of rows.
        // Is always in 0..row_len
        let pos = (ordered_index * parallel_encoding_units) % row_len;

        // Offset of the position within a chunk of rows
        // Is always in 0..parallel_encoding_units
        let offset = (ordered_index / img_width) % parallel_encoding_units;

        let unordered_index = chunk + pos + offset;
        ordered_vec.push(img[unordered_index])
    }

    ordered_vec
}

#[test]
fn test_reorder_img() {
    // One Row
    let ordered = vec![0, 1, 2, 3];
    assert_eq!(reorder_img(ordered.clone(), 1, 4), ordered);

    // Two rows but `img_width=1`
    let ordered = vec![0, 1];
    assert_eq!(reorder_img(ordered.clone(), 2, 1), ordered);

    // 4 rows but `img_width=1`
    let ordered = vec![0, 1, 2, 3];
    assert_eq!(reorder_img(ordered.clone(), 2, 1), ordered);

    // Actually ordering stuff
    let ordered = vec![0, 1, 2, 3];
    let unordered = vec![0, 2, 1, 3];
    assert_eq!(reorder_img(unordered, 2, 2), ordered)
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
            Coding::Unary => {
                // The current bit (=bool) is part of the unary coding
                match bool {
                    true => state.unary += 1,                  // Unary coding continues
                    false => state.coding = Coding::Remainder, // End of unary coding
                }
            }
            Coding::Remainder => {
                // The current bit (=bool) is part of the unary coding

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
    let code_0 = vec![false, false, false]; // == 0
    let code_3 = vec![false, true, true]; // == 3
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

    assert_eq!(decode_rice(&code, 4, 2), vec![0, 3, 7, 11])
}
