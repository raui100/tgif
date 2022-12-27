use std::io::Write;

use log::{debug, info};
use ndarray::Axis;
use nshare::ToNdarray2;

use crate::args::Args;
use crate::constants::{CHUNK_SIZE, POW_OF_TWO};

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
                parallel_encoding_units,
            ]
                .into_iter()
                .flat_map(|v| v.to_be_bytes())
        );
        img_u8.push(remainder_bits);
    }

    let header_len = img_u8.len();
    dbg!(header_len);

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

    let img_len = img_u8.len() - header_len;
    dbg!(img_len);

    debug!("Writing the image to disk");
    let mut file = std::fs::File::create(&args.dst).expect("Failed creating destination file");
    file.write_all(&img_u8)
        .expect("Failed writing the image to disk");

    info!("Finished!")
}


fn rice_code(image: &ndarray::Array2<u8>, parallel_encoding_units: u32, remainder_bits: u8) -> Vec<bool> {
    // Asserting that the image dimensions are compatible with the number of parallel encoding units
    debug_assert_eq!(
        image.shape()[0] % parallel_encoding_units as usize,
        0,
        "Number of parallel encoding units and image height don't match"
    );

    // The remainder is smaller than this number `assert!(rem < rem_max)
    let rem_max = 2_u8.pow(remainder_bits as u32);

    let mut chunk: usize = 0;
    let mut wasted: usize = 0;
    let size = image.shape()[0] * image.shape()[1];

    // Stores the encoded image as a vector of bool
    let mut img: Vec<bool> = Vec::with_capacity(
        8 * size, // Capacity is estimated for no compression
    );

    // Iterating over the image
    debug!("Encoding the image as Vec<bool>");
    for axis in image.axis_iter(Axis(0)) {
        let mut prev: u8 = 0;
        for pixel in axis {
            let delta = prev.wrapping_sub(*pixel);
            let rice = rice_index(delta);
            let quot = rice / rem_max;
            let rem = rice % rem_max;
            prev = *pixel;  // Updating the previous pixel
            let bits = quot as usize + 1 + remainder_bits as usize;

            if chunk + bits > CHUNK_SIZE {
                wasted += CHUNK_SIZE - chunk;
                img.extend(vec![true; CHUNK_SIZE - chunk]);
                chunk = 0;
            }

            chunk += bits;
            unary_coding(&mut img, quot);  // Unary coding of the quotient
            remainder_coding(&mut img, rem, remainder_bits);  // Binary coding of the rem
        }
    }
    info!("Used {:.2} % Bits for padding", 100.0 * (wasted as f64 / size as f64));
    img
}

/// Codes the remainder as boolean binary with `remainder_bits` bit-width
fn remainder_coding(img: &mut Vec<bool>, rem: u8, rem_bits: u8) {
    debug_assert!(rem_bits <= 8);  // Hoping for better optimization
    debug_assert!(rem < 2u8.pow(rem_bits as u32));
    img.extend(
        (0..rem_bits)
            .rev() // <-> Most significant bit
            .map(|ind| rem & POW_OF_TWO[ind as usize] != 0)
    )
}

/// Unary coding of the quotient
fn unary_coding(img: &mut Vec<bool>, quot: u8) {
    img.extend(vec![true; quot as usize]);
    img.push(false);
}

/// Calculates the rice index for a given number
fn rice_index(num: u8) -> u8 {
    if num <= 127 { num * 2 }
    else { (u8::MAX - num) * 2 + 1 }
}

#[test]
fn test_rice_index() {
    for num in 0..=u8::MAX {
        assert_eq!(
            rice_index(num),
            {  // Alternative implementation to calculate the rice index
                // Casts eg: 255 -> -1 and then casts to i16 prevents overflow
                let num = (num as i8) as i16;
                if num >= 0 { (num * 2) as u8 } else { (-num * 2 - 1) as u8 }
            }
        )
    }
}
