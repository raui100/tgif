use log::{debug, info, trace};

use crate::args::Args;
use crate::constants::{CHUNK_SIZE, U8_TO_ARRAY_BOOL};

pub fn decode(args: &Args) {
    debug!("Reading the TGIF file from disk");
    let tgif = std::fs::read(&args.src)
        .unwrap_or_else(|_| panic!("Failed reading {}", &args.src));

    debug!("Parsing the header");
    let header = parse_header(&tgif, args);
    dbg!(&header);
    debug!("Decoding Rice-code to Rice-index");
    let ordered_rice_indices = decode_rice(
        &tgif[header.start_index..],
        header.remainder,
        (header.width * header.height) as usize,
    );

    debug!("Transforming the Rice-Index to deltas");
    let deltas = reverse_rice_index(ordered_rice_indices);

    debug!("Transforming the delta back to the original image");
    let img_u8 = reverse_delta(
        deltas,
        header.width as usize,
    );

    debug!("Saving the original image to disk");
    image::save_buffer(
        &args.dst,
        &img_u8,
        header.width,
        header.height,
        image::ColorType::L8,
    )
        .unwrap();

    info!("Finished!")
}

/// Reverse index
fn reverse_rice_index(mut vec: Vec<u8>) -> Vec<u8> {
    for num in vec.iter_mut() {
        *num = rev_rice_index(*num)
    }

    vec
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
fn reverse_delta(mut delta: Vec<u8>, img_width: usize) -> Vec<u8> {
    for chunk in delta.chunks_exact_mut(img_width) {
        let mut prev_num = 0_u8;
        for delta in chunk.iter_mut() {
            prev_num = prev_num.wrapping_sub(*delta);
            *delta = prev_num
        }
    }

    delta
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


fn decode_rice(img: &[u8], remainder_bits: u8, size: usize) -> Vec<u8> {
    let mut img_code_u8 = Vec::with_capacity(size);
    dbg!(remainder_bits);

    for chunk in img.chunks(CHUNK_SIZE / 8) {
        let mut num = 0_u8;
        let mut remainder_index = 0_u8;
        let mut code_unary = true;
        for number in chunk {
            for bit in U8_TO_ARRAY_BOOL[*number as usize] {
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
                    }
                }
                if !code_unary && remainder_index == remainder_bits {
                    img_code_u8.push(num);
                    num = 0;
                    remainder_index = 0;
                    code_unary = true;
                }
            }
        }
    }
    // img_code_u8.extend(vec![0u8; 26]);
    dbg!(size, img_code_u8.len());
    img_code_u8
}

#[derive(Debug, Clone)]
struct Header {
    _name: String,
    width: u32,
    height: u32,
    pae: u32,
    remainder: u8,
    start_index: usize,
}

/// Parses the header from the beginning of the file or from CLI `args`
fn parse_header(img: &[u8], args: &Args) -> Header {
    if args.no_header {
        // Image has no header. Using CLI arguments
        trace!("Image has no header. Using CLI arguments");
        Header {
            _name: "TGIF".to_string(),
            width: args.width.expect("Image width is missing"),
            height: args.height.expect("Image height is missing"),
            pae: args.parallel_encoding_units.expect("Number of parallel encoding units is missing"),
            remainder: args.remainder_bits.expect("Bit size of remainder is missing"),
            start_index: 0,  // Without header the data starts with the first bit
        }
    } else {
        trace!("Reading header from image");
        Header {
            _name: "TGIF".to_string(),
            width: img[4..8].iter().fold(0_u32, |res, val| (res << 8) + (*val as u32)),
            height: img[8..12].iter().fold(0_u32, |res, val| (res << 8) + (*val as u32)),
            pae: img[12..16].iter().fold(0_u32, |res, val| (res << 8) + (*val as u32)),
            remainder: img[16],
            start_index: 17,
        }
    }
}
