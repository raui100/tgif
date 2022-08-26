use log::{debug, info};

use crate::args::Args;
use crate::codebooks::_vec_bool_to_u8;

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
        .flat_map(|num| u8_to_array_bool(*num))
        .collect();

    debug!("Decoding Rice-code to Rice-index");
    let unordered_rice_indices = decode_rice(
        img_code_bool,
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
    vec.into_iter().map(rev_rice_index).collect()
}

#[inline(always)]
fn rev_rice_index(index: u8) -> u8 { REV_RICE_INDEX[index as usize] }

const REV_RICE_INDEX: [u8; 256] = [0, 255, 1, 254, 2, 253, 3, 252, 4, 251, 5, 250, 6, 249, 7, 248, 8, 247, 9, 246, 10, 245, 11, 244, 12, 243, 13, 242, 14, 241, 15, 240, 16, 239, 17, 238, 18, 237, 19, 236, 20, 235, 21, 234, 22, 233, 23, 232, 24, 231, 25, 230, 26, 229, 27, 228, 28, 227, 29, 226, 30, 225, 31, 224, 32, 223, 33, 222, 34, 221, 35, 220, 36, 219, 37, 218, 38, 217, 39, 216, 40, 215, 41, 214, 42, 213, 43, 212, 44, 211, 45, 210, 46, 209, 47, 208, 48, 207, 49, 206, 50, 205, 51, 204, 52, 203, 53, 202, 54, 201, 55, 200, 56, 199, 57, 198, 58, 197, 59, 196, 60, 195, 61, 194, 62, 193, 63, 192, 64, 191, 65, 190, 66, 189, 67, 188, 68, 187, 69, 186, 70, 185, 71, 184, 72, 183, 73, 182, 74, 181, 75, 180, 76, 179, 77, 178, 78, 177, 79, 176, 80, 175, 81, 174, 82, 173, 83, 172, 84, 171, 85, 170, 86, 169, 87, 168, 88, 167, 89, 166, 90, 165, 91, 164, 92, 163, 93, 162, 94, 161, 95, 160, 96, 159, 97, 158, 98, 157, 99, 156, 100, 155, 101, 154, 102, 153, 103, 152, 104, 151, 105, 150, 106, 149, 107, 148, 108, 147, 109, 146, 110, 145, 111, 144, 112, 143, 113, 142, 114, 141, 115, 140, 116, 139, 117, 138, 118, 137, 119, 136, 120, 135, 121, 134, 122, 133, 123, 132, 124, 131, 125, 130, 126, 129, 127, 128];

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


#[inline(always)]
fn u8_to_array_bool(num: u8) -> [bool; 8] {
    U8_TO_ARRAY_BOOL[num as usize]
}

const U8_TO_ARRAY_BOOL: [[bool; 8]; 256] = [[false, false, false, false, false, false, false, false], [false, false, false, false, false, false, false, true], [false, false, false, false, false, false, true, false], [false, false, false, false, false, false, true, true], [false, false, false, false, false, true, false, false], [false, false, false, false, false, true, false, true], [false, false, false, false, false, true, true, false], [false, false, false, false, false, true, true, true], [false, false, false, false, true, false, false, false], [false, false, false, false, true, false, false, true], [false, false, false, false, true, false, true, false], [false, false, false, false, true, false, true, true], [false, false, false, false, true, true, false, false], [false, false, false, false, true, true, false, true], [false, false, false, false, true, true, true, false], [false, false, false, false, true, true, true, true], [false, false, false, true, false, false, false, false], [false, false, false, true, false, false, false, true], [false, false, false, true, false, false, true, false], [false, false, false, true, false, false, true, true], [false, false, false, true, false, true, false, false], [false, false, false, true, false, true, false, true], [false, false, false, true, false, true, true, false], [false, false, false, true, false, true, true, true], [false, false, false, true, true, false, false, false], [false, false, false, true, true, false, false, true], [false, false, false, true, true, false, true, false], [false, false, false, true, true, false, true, true], [false, false, false, true, true, true, false, false], [false, false, false, true, true, true, false, true], [false, false, false, true, true, true, true, false], [false, false, false, true, true, true, true, true], [false, false, true, false, false, false, false, false], [false, false, true, false, false, false, false, true], [false, false, true, false, false, false, true, false], [false, false, true, false, false, false, true, true], [false, false, true, false, false, true, false, false], [false, false, true, false, false, true, false, true], [false, false, true, false, false, true, true, false], [false, false, true, false, false, true, true, true], [false, false, true, false, true, false, false, false], [false, false, true, false, true, false, false, true], [false, false, true, false, true, false, true, false], [false, false, true, false, true, false, true, true], [false, false, true, false, true, true, false, false], [false, false, true, false, true, true, false, true], [false, false, true, false, true, true, true, false], [false, false, true, false, true, true, true, true], [false, false, true, true, false, false, false, false], [false, false, true, true, false, false, false, true], [false, false, true, true, false, false, true, false], [false, false, true, true, false, false, true, true], [false, false, true, true, false, true, false, false], [false, false, true, true, false, true, false, true], [false, false, true, true, false, true, true, false], [false, false, true, true, false, true, true, true], [false, false, true, true, true, false, false, false], [false, false, true, true, true, false, false, true], [false, false, true, true, true, false, true, false], [false, false, true, true, true, false, true, true], [false, false, true, true, true, true, false, false], [false, false, true, true, true, true, false, true], [false, false, true, true, true, true, true, false], [false, false, true, true, true, true, true, true], [false, true, false, false, false, false, false, false], [false, true, false, false, false, false, false, true], [false, true, false, false, false, false, true, false], [false, true, false, false, false, false, true, true], [false, true, false, false, false, true, false, false], [false, true, false, false, false, true, false, true], [false, true, false, false, false, true, true, false], [false, true, false, false, false, true, true, true], [false, true, false, false, true, false, false, false], [false, true, false, false, true, false, false, true], [false, true, false, false, true, false, true, false], [false, true, false, false, true, false, true, true], [false, true, false, false, true, true, false, false], [false, true, false, false, true, true, false, true], [false, true, false, false, true, true, true, false], [false, true, false, false, true, true, true, true], [false, true, false, true, false, false, false, false], [false, true, false, true, false, false, false, true], [false, true, false, true, false, false, true, false], [false, true, false, true, false, false, true, true], [false, true, false, true, false, true, false, false], [false, true, false, true, false, true, false, true], [false, true, false, true, false, true, true, false], [false, true, false, true, false, true, true, true], [false, true, false, true, true, false, false, false], [false, true, false, true, true, false, false, true], [false, true, false, true, true, false, true, false], [false, true, false, true, true, false, true, true], [false, true, false, true, true, true, false, false], [false, true, false, true, true, true, false, true], [false, true, false, true, true, true, true, false], [false, true, false, true, true, true, true, true], [false, true, true, false, false, false, false, false], [false, true, true, false, false, false, false, true], [false, true, true, false, false, false, true, false], [false, true, true, false, false, false, true, true], [false, true, true, false, false, true, false, false], [false, true, true, false, false, true, false, true], [false, true, true, false, false, true, true, false], [false, true, true, false, false, true, true, true], [false, true, true, false, true, false, false, false], [false, true, true, false, true, false, false, true], [false, true, true, false, true, false, true, false], [false, true, true, false, true, false, true, true], [false, true, true, false, true, true, false, false], [false, true, true, false, true, true, false, true], [false, true, true, false, true, true, true, false], [false, true, true, false, true, true, true, true], [false, true, true, true, false, false, false, false], [false, true, true, true, false, false, false, true], [false, true, true, true, false, false, true, false], [false, true, true, true, false, false, true, true], [false, true, true, true, false, true, false, false], [false, true, true, true, false, true, false, true], [false, true, true, true, false, true, true, false], [false, true, true, true, false, true, true, true], [false, true, true, true, true, false, false, false], [false, true, true, true, true, false, false, true], [false, true, true, true, true, false, true, false], [false, true, true, true, true, false, true, true], [false, true, true, true, true, true, false, false], [false, true, true, true, true, true, false, true], [false, true, true, true, true, true, true, false], [false, true, true, true, true, true, true, true], [true, false, false, false, false, false, false, false], [true, false, false, false, false, false, false, true], [true, false, false, false, false, false, true, false], [true, false, false, false, false, false, true, true], [true, false, false, false, false, true, false, false], [true, false, false, false, false, true, false, true], [true, false, false, false, false, true, true, false], [true, false, false, false, false, true, true, true], [true, false, false, false, true, false, false, false], [true, false, false, false, true, false, false, true], [true, false, false, false, true, false, true, false], [true, false, false, false, true, false, true, true], [true, false, false, false, true, true, false, false], [true, false, false, false, true, true, false, true], [true, false, false, false, true, true, true, false], [true, false, false, false, true, true, true, true], [true, false, false, true, false, false, false, false], [true, false, false, true, false, false, false, true], [true, false, false, true, false, false, true, false], [true, false, false, true, false, false, true, true], [true, false, false, true, false, true, false, false], [true, false, false, true, false, true, false, true], [true, false, false, true, false, true, true, false], [true, false, false, true, false, true, true, true], [true, false, false, true, true, false, false, false], [true, false, false, true, true, false, false, true], [true, false, false, true, true, false, true, false], [true, false, false, true, true, false, true, true], [true, false, false, true, true, true, false, false], [true, false, false, true, true, true, false, true], [true, false, false, true, true, true, true, false], [true, false, false, true, true, true, true, true], [true, false, true, false, false, false, false, false], [true, false, true, false, false, false, false, true], [true, false, true, false, false, false, true, false], [true, false, true, false, false, false, true, true], [true, false, true, false, false, true, false, false], [true, false, true, false, false, true, false, true], [true, false, true, false, false, true, true, false], [true, false, true, false, false, true, true, true], [true, false, true, false, true, false, false, false], [true, false, true, false, true, false, false, true], [true, false, true, false, true, false, true, false], [true, false, true, false, true, false, true, true], [true, false, true, false, true, true, false, false], [true, false, true, false, true, true, false, true], [true, false, true, false, true, true, true, false], [true, false, true, false, true, true, true, true], [true, false, true, true, false, false, false, false], [true, false, true, true, false, false, false, true], [true, false, true, true, false, false, true, false], [true, false, true, true, false, false, true, true], [true, false, true, true, false, true, false, false], [true, false, true, true, false, true, false, true], [true, false, true, true, false, true, true, false], [true, false, true, true, false, true, true, true], [true, false, true, true, true, false, false, false], [true, false, true, true, true, false, false, true], [true, false, true, true, true, false, true, false], [true, false, true, true, true, false, true, true], [true, false, true, true, true, true, false, false], [true, false, true, true, true, true, false, true], [true, false, true, true, true, true, true, false], [true, false, true, true, true, true, true, true], [true, true, false, false, false, false, false, false], [true, true, false, false, false, false, false, true], [true, true, false, false, false, false, true, false], [true, true, false, false, false, false, true, true], [true, true, false, false, false, true, false, false], [true, true, false, false, false, true, false, true], [true, true, false, false, false, true, true, false], [true, true, false, false, false, true, true, true], [true, true, false, false, true, false, false, false], [true, true, false, false, true, false, false, true], [true, true, false, false, true, false, true, false], [true, true, false, false, true, false, true, true], [true, true, false, false, true, true, false, false], [true, true, false, false, true, true, false, true], [true, true, false, false, true, true, true, false], [true, true, false, false, true, true, true, true], [true, true, false, true, false, false, false, false], [true, true, false, true, false, false, false, true], [true, true, false, true, false, false, true, false], [true, true, false, true, false, false, true, true], [true, true, false, true, false, true, false, false], [true, true, false, true, false, true, false, true], [true, true, false, true, false, true, true, false], [true, true, false, true, false, true, true, true], [true, true, false, true, true, false, false, false], [true, true, false, true, true, false, false, true], [true, true, false, true, true, false, true, false], [true, true, false, true, true, false, true, true], [true, true, false, true, true, true, false, false], [true, true, false, true, true, true, false, true], [true, true, false, true, true, true, true, false], [true, true, false, true, true, true, true, true], [true, true, true, false, false, false, false, false], [true, true, true, false, false, false, false, true], [true, true, true, false, false, false, true, false], [true, true, true, false, false, false, true, true], [true, true, true, false, false, true, false, false], [true, true, true, false, false, true, false, true], [true, true, true, false, false, true, true, false], [true, true, true, false, false, true, true, true], [true, true, true, false, true, false, false, false], [true, true, true, false, true, false, false, true], [true, true, true, false, true, false, true, false], [true, true, true, false, true, false, true, true], [true, true, true, false, true, true, false, false], [true, true, true, false, true, true, false, true], [true, true, true, false, true, true, true, false], [true, true, true, false, true, true, true, true], [true, true, true, true, false, false, false, false], [true, true, true, true, false, false, false, true], [true, true, true, true, false, false, true, false], [true, true, true, true, false, false, true, true], [true, true, true, true, false, true, false, false], [true, true, true, true, false, true, false, true], [true, true, true, true, false, true, true, false], [true, true, true, true, false, true, true, true], [true, true, true, true, true, false, false, false], [true, true, true, true, true, false, false, true], [true, true, true, true, true, false, true, false], [true, true, true, true, true, false, true, true], [true, true, true, true, true, true, false, false], [true, true, true, true, true, true, false, true], [true, true, true, true, true, true, true, false], [true, true, true, true, true, true, true, true], ];


#[test]
fn test_u8_to_array_bool() {
    assert_eq!(
        u8_to_array_bool(0),
        [false, false, false, false, false, false, false, false, ]
    );

    assert_eq!(
        u8_to_array_bool(255),
        [true, true, true, true, true, true, true, true, ]
    );

    assert_eq!(
        u8_to_array_bool(1),
        [false, false, false, false, false, false, false, true]
    );

    assert_eq!(
        u8_to_array_bool(254),
        [true, true, true, true, true, true, true, false]
    );

    assert_eq!(
        u8_to_array_bool(128),
        [true, false, false, false, false, false, false, false]
    );
}


fn _decode_rice(code: Vec<bool>, remainder_bits: u8, pixel_count: usize) -> Vec<u8> {
    debug!("Creating the codebook");
    let code_book = _vec_bool_to_u8(
        remainder_bits,
        2_u8.pow(remainder_bits as u32),
    );

    assert!(remainder_bits <= 7);
    let remainder_bits: usize = remainder_bits as usize;

    debug!("Decoding the Rice-code");
    let mut start = 0;
    let mut ind = 0;
    let mut img_code_u8 = Vec::new();
    loop {
        match code[ind] {
            true => ind += 1,
            false => {
                ind += remainder_bits + 1;
                let num = code_book.get(&code[start..ind]).unwrap();
                img_code_u8.push(*num);
                start = ind;

                if img_code_u8.len() == pixel_count { return img_code_u8 }
            }
        }
    }
}

fn decode_rice(code: Vec<bool>, remainder_bits: u8)-> Vec<u8> {
    let mut img_code_u8 = Vec::with_capacity(code.len() / 2);
    let mut num = 0_u8;
    let mut remainder_index = 0_u8;
    let mut code_unary = true;

    // The algorithm can be simplified if remainder_bits == 0
    if remainder_bits == 0 {
        for bit in code {
            match bit {
                true => num += 1,
                false => {
                    img_code_u8.push(num);
                    num = 0;
                }
            }
        }
        return img_code_u8;
    }

    // General algorithm for remainder_bits >= 1
    for bit in code {
        match code_unary {
            true => {
                match bit {
                    true => num += 1,
                    false => code_unary = false,
                }
            }
            false => {
                num = (num << 1) + (bit as u8);
                remainder_index += 1;
                if remainder_index == remainder_bits {
                    img_code_u8.push(num);
                    num = 0;
                    remainder_index = 0;
                    code_unary = true;
                }
            }
        }
    }

    img_code_u8
}

#[test]
fn test_decode_rice() {
    let code_0 = vec![false, false, false]; // == 0
    let code_3 = vec![false, true, true]; // == 3
    let code_7 = vec![true, false, true, true]; // == 7
    let code_11 = vec![true, true, false, true, true]; // == 11
    let n = [0, 3, 7, 11];

    // Testing single numbers
    assert_eq!(decode_rice(code_0.clone(), 2), vec![n[0]]);
    assert_eq!(decode_rice(code_3.clone(), 2), vec![n[1]]);
    assert_eq!(decode_rice(code_7.clone(), 2), vec![n[2]]);
    assert_eq!(decode_rice(code_11.clone(), 2), vec![n[3]]);

    // Testing "real" vectors
    let code: Vec<bool> = [code_0, code_3, code_7, code_11]
        .iter()
        .flat_map(|v| v.clone())
        .collect();

    assert_eq!(decode_rice(code, 2), n)
}
