use std::io::Write;

use log::{debug, info};
use ndarray::Axis;
use nshare::ToNdarray2;

use crate::args::Args;

/// Encodes an image to TGIF
pub fn encode(args: &Args) {
    let parallel_encoding_units = args.parallel_encoding_units.expect("Check for `None` beforehand!");
    let remainder_bits = args.remainder_bits.expect("Check for `None` beforehand!");

    debug!("Reading the image from disk and converting it into an 2D ndarray");
    let image = image::open(&args.src)
        .expect("Failed reading input file.")
        .to_luma8()
        .into_ndarray2();

    assert_eq!(
        image.shape()[0] % parallel_encoding_units as usize,
        0,
        "Number of parallel encoding units and image height don't match"
    );

    let mut img_bool = rice_code(&image, parallel_encoding_units, remainder_bits);

    // Padding the end with "1"
    img_bool.extend(vec![true; 8 - (img_bool.len() % 8)]);
    debug!("Finished encoding to TGIF");

    // Calculating the compression rate
    let compression_rate = (img_bool.len() / 8) as f64
        / (image.shape()[0] * image.shape()[1]) as f64;
    info!("Achieved {compression_rate:.4} compression rate for {:#?}", &args.src );

    // Building a vector with enough capacity for the header and the encoded image
    let mut img_u8: Vec<u8> = Vec::with_capacity(img_bool.len() / 8 + 4);

    // Creating the header
    if !args.no_header {
        debug!("Creating the header");
        // The header has these 4 byte wide entries:
        // 1. The name of the format: TGIF
        // 2. The image width as u32
        // 3. The image height as u32
        // 4. Number of parallel encoding units that have been used during encoding
        // 5. The number of bits the remainder is represented as
        img_u8.extend(
            [
                u32::from_be_bytes(*b"TGIF"),
                image.shape()[1] as u32,
                image.shape()[0] as u32,
                parallel_encoding_units as u32,
            ]
                .into_iter()
                .flat_map(|v| v.to_be_bytes())
                .collect::<Vec<u8>>(),
        );
        img_u8.push(remainder_bits);
    }

    debug!("Encoding the image as Vec<u8>");
    img_u8.extend(
        img_bool
            .chunks_exact(8)
            .map(|chunk| {
                chunk
                    .iter()
                    .fold(0_u8, |value, bool| (value << 1) + (*bool as u8))
            })
            .collect::<Vec<u8>>(),
    );

    debug!("Writing the image to disk");
    let mut file = std::fs::File::create(&args.dst).expect("Failed creating destination file");
    file.write_all(&img_u8)
        .expect("Failed writing the image to disk");

    info!("Finished!")
}

#[inline(always)]
pub fn rice_index(delta: u8) -> u8 { RICE_INDEX[delta as usize] }

const RICE_INDEX: [u8; 256] = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 52, 54, 56, 58, 60, 62, 64, 66, 68, 70, 72, 74, 76, 78, 80, 82, 84, 86, 88, 90, 92, 94, 96, 98, 100, 102, 104, 106, 108, 110, 112, 114, 116, 118, 120, 122, 124, 126, 128, 130, 132, 134, 136, 138, 140, 142, 144, 146, 148, 150, 152, 154, 156, 158, 160, 162, 164, 166, 168, 170, 172, 174, 176, 178, 180, 182, 184, 186, 188, 190, 192, 194, 196, 198, 200, 202, 204, 206, 208, 210, 212, 214, 216, 218, 220, 222, 224, 226, 228, 230, 232, 234, 236, 238, 240, 242, 244, 246, 248, 250, 252, 254, 255, 253, 251, 249, 247, 245, 243, 241, 239, 237, 235, 233, 231, 229, 227, 225, 223, 221, 219, 217, 215, 213, 211, 209, 207, 205, 203, 201, 199, 197, 195, 193, 191, 189, 187, 185, 183, 181, 179, 177, 175, 173, 171, 169, 167, 165, 163, 161, 159, 157, 155, 153, 151, 149, 147, 145, 143, 141, 139, 137, 135, 133, 131, 129, 127, 125, 123, 121, 119, 117, 115, 113, 111, 109, 107, 105, 103, 101, 99, 97, 95, 93, 91, 89, 87, 85, 83, 81, 79, 77, 75, 73, 71, 69, 67, 65, 63, 61, 59, 57, 55, 53, 51, 49, 47, 45, 43, 41, 39, 37, 35, 33, 31, 29, 27, 25, 23, 21, 19, 17, 15, 13, 11, 9, 7, 5, 3, 1];
#[test]
fn test_rice_index() {
    assert_eq!(rice_index(0), 0);
    assert_eq!(rice_index(1), 2);
    assert_eq!(rice_index(2), 4);
    assert_eq!(rice_index(255), 1);
    assert_eq!(rice_index(254), 3);
    assert_eq!(rice_index(127), 254);
    assert_eq!(rice_index(128), 255);
}

fn rice_code(image: &ndarray::Array2<u8>, parallel_encoding_units: u32, remainder_bits: u8) -> Vec<bool> {
    // Asserting that the image dimensions are compatible with the number of parallel encoding units
    debug_assert_eq!(
        image.shape()[0] % parallel_encoding_units as usize,
        0,
        "Number of parallel encoding units and image height don't match"
    );

    // Numbers that are being used a million times and stored here for performance
    let chunk_length = image.shape()[1] * parallel_encoding_units as usize;
    let remainder_range = 2_u8.pow(remainder_bits as u32);
    let prev_pixels_reset = vec![0_u8; parallel_encoding_units as usize];

    // Stores the encoded image as a vector of bool
    let mut img_bool: Vec<bool> = Vec::with_capacity(
        8 * image.shape()[0] * image.shape()[1], // Capacity is estimated for no compression
    );

    // Iterating over the image
    debug!("Encoding the image as Vec<bool>");
    for rows in image.axis_chunks_iter(Axis(0), parallel_encoding_units as usize) {
        let mut prev_pixels = prev_pixels_reset.clone();
        for index in 0..chunk_length {
            let pos_x = index / parallel_encoding_units as usize;
            let pos_y = index % parallel_encoding_units as usize;
            let prev_pixel = &prev_pixels[pos_y];
            let pixel = rows[(pos_y, pos_x)];
            let delta = prev_pixel.wrapping_sub(pixel);
            let index = rice_index(delta);
            let unary = index >> remainder_bits;
            let remainder = index & (remainder_range - 1);
            prev_pixels[pos_y] = pixel;

            // Unary coding. Pushes n "1" and one "0" to the vec
            img_bool.extend(vec![true; unary as usize]);
            img_bool.push(false);

            // // Pushing the remainder to the vec
            img_bool.extend(
                (0..remainder_bits)
                .rev() // <-> Most significant bit
                .map(|ind| remainder & POW_OF_TWO[ind as usize] != 0)
            );
        }
    }

    img_bool
}

#[test]
fn test_rice_code() {
    // // Most simple case
    // let arr = ndarray::array![[0, 0], [0, 0]];
    // assert_eq!(
    //     rice_code(&arr, 1, 0),
    //     vec![false, false, false, false]
    // );

    // Simple case
    let arr = ndarray::array![[0, 255], [0, 254]];
    assert_eq!(
        rice_code(&arr, 1, 2),
        vec![
            false, false, false,
            false, true, false,
            false, false, false,
            true, false, false, false,
        ]
    );

    // Simple case with two parallel encoding units
    let arr = ndarray::array![[0, 255], [0, 254]];
    assert_eq!(
        rice_code(&arr, 2, 2),
        vec![
            false, false, false, false, false, false, false, true, false, true, false, false, false,
        ]
    );

    // Edge case with one pixel
    let arr = ndarray::array![[255]];
    assert_eq!(rice_code(&arr, 1, 2), vec![false, true, false]);
}

const POW_OF_TWO: [u8; 8] = [1, 2, 4, 8, 16, 32, 64, 128];

pub fn remainder_coding(remainder: u8, remainder_bits: u8) -> Vec<bool> {
    (0..remainder_bits)
        .rev() // <-> Most significant bit
        .map(|ind| remainder & POW_OF_TWO[ind as usize] != 0)
        .collect()
}

#[test]
fn test_remainder_coding() {
    assert_eq!(remainder_coding(0, 1), vec![false]);
    assert_eq!(remainder_coding(1, 1), vec![true]);
    assert_eq!(remainder_coding(0, 2), vec![false, false]);
    assert_eq!(remainder_coding(1, 2), vec![false, true]);
    assert_eq!(remainder_coding(2, 2), vec![true, false]);
    assert_eq!(remainder_coding(3, 2), vec![true, true]);
}

#[inline(always)]
pub fn unary_coding(num: u8) -> Vec<bool> {
    let mut code = vec![true; num as usize];
    code.push(false);

    code
}

#[test]
fn test_unary_coding() {
    for ind in 0..=u8::MAX {
        let code = unary_coding(ind);
        for bit in code[..ind as usize].iter() {
            assert!(bit)
        }
        assert!(!code.last().unwrap())
    }
}

#[inline(always)]
pub fn u8_to_rice_code(num: u8, remainder_bits: u8, remainder_range: u8) -> Vec<bool> {
    debug_assert_eq!(remainder_range, 2u8.pow(remainder_bits as u32));
    let unary = num >> remainder_bits;
    let mut code = unary_coding(unary);
    if remainder_bits >= 1 {
        let remainder = remainder_coding(
            num % remainder_range,
            remainder_bits,
        );
        code.extend(remainder);
    }

    code
}

#[test]
fn test_u8_to_rice_code() {
    assert_eq!(
        u8_to_rice_code(0, 0, POW_OF_TWO[0]),
        vec![false]
    );

    assert_eq!(
        u8_to_rice_code(0, 1, POW_OF_TWO[1]),
        vec![false, false]
    );

    assert_eq!(
        u8_to_rice_code(1, 0, POW_OF_TWO[0]),
        vec![true, false]
    );

    assert_eq!(
        u8_to_rice_code(1, 1, POW_OF_TWO[1]),
        vec![false, true]
    );

    assert_eq!(
        u8_to_rice_code(4, 2, POW_OF_TWO[2]),
        vec![true, false, false, false]
    );
}


