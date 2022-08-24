use log::{debug, info};
use ndarray::Axis;
use nshare::ToNdarray2;

use crate::args::Args;

/// Encodes an image to TGIF
pub fn encode(args: &Args, parallel_units: u32, remainder_bits: u8) -> Vec<u8> {
    debug!("Reading the image from disk and converting it into an 2D ndarray");
    let image: ndarray::Array2<u8> = image::open(&args.src)
        .expect("Failed reading input file.")
        .to_luma8()
        .into_ndarray2();

    let mut img_bool = rice_code(&image, parallel_units, remainder_bits);

    // Padding the end with "0"
    img_bool.extend(vec![false; 8 - (img_bool.len() % 8)]);
    debug!("Finished encoding to TGIF");

    // Calculating the compression rate
    let compression_rate = (img_bool.len() / 8) as f64
        / (image.shape()[0] * image.shape()[1]) as f64;
    info!("Achieved {compression_rate:.4} compression rate for {:#?}", &args.src);

    // Building a vector with enough capacity for the header and the encoded image
    let mut img_u8: Vec<u8> = Vec::with_capacity(img_bool.len() / 8 + 4);

    // Creating the header
    if !args.no_header {
        debug!("Creating the header");
        // The header has these 4 byte wide entries:
        // 1. The name of the format: TGIF
        // 2. The image width as u32
        // 3. The image height as u32
        // 4. Number of parallel units that have been used during encoding
        // 5. The number of bits the remainder is represented as
        img_u8.extend([
            u32::from_be_bytes(*b"TGIF"),
            image.shape()[1] as u32,
            image.shape()[0] as u32,
            parallel_units as u32,
        ].into_iter()
            .flat_map(|v| v.to_be_bytes())
            .collect::<Vec<u8>>()
        );
        img_u8.push(remainder_bits);
    }

    debug!("Encoding the image as Vec<u8>");
    img_u8.extend(
        img_bool.chunks_exact(8)
            .map(|chunk| chunk.iter()
                .fold(0_u8, |value, bool| (value << 1) + (*bool as u8))
            )
            .collect::<Vec<u8>>()
    );

    img_u8
}

fn rice_index(delta: u8) -> u8 {
    if delta <= i8::MAX as u8 {
        delta * 2
    } else {
        (i8::from_be_bytes(delta.to_be_bytes()) as i16 * (-2) - 1).try_into().unwrap()
    }
}

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

fn rice_code(image: &ndarray::Array2<u8>, parallel_units: u32, remainder_bits: u8) -> Vec<bool> {
    // Asserting that the image dimensions are compatible with the number of parallel units
    assert_eq!(
        image.shape()[0] % parallel_units as usize,
        0,
        "Number of parallel units and image height don't match"
    );


    // Numbers that are being used a million times and stored here for performance
    let chunk_length = image.shape()[1] * parallel_units as usize;
    let remainder_bits_square = remainder_bits.pow(2);
    let prev_pixels_reset = vec![0_u8; parallel_units as usize];


    // Stores the encoded image as a vector of bool
    let mut img_bool: Vec<bool> = Vec::with_capacity(
        8 * image.shape()[0] * image.shape()[1]  // Capacity is estimated for no compression
    );

    // Iterating over the image
    debug!("Encoding the image as Vec<bool>");

    for rows in image.axis_chunks_iter(Axis(0), parallel_units as usize) {
        let mut prev_pixels = prev_pixels_reset.clone();
        for index in 0..chunk_length {
            let pos_x = index / parallel_units as usize;
            let pos_y = index % parallel_units as usize;
            let prev_pixel = &prev_pixels[pos_y];
            let pixel = rows.get((pos_y, pos_x)).unwrap();
            let delta = prev_pixel.wrapping_sub(*pixel);
            let index = rice_index(delta);
            let unary = index / remainder_bits_square;
            let remainder = index % remainder_bits_square;
            prev_pixels[pos_y] = *pixel;

            // Unary coding. Pushes n "1" and one "0" to the vec
            img_bool.extend(vec![true; unary as usize]);
            img_bool.push(false);

            // Pushing the remainder to the vec
            assert_eq!(remainder_bits, 2, "Only implemented for two bit remainders");
            match remainder {
                0 => img_bool.extend([false, false]),
                1 => img_bool.extend([false, true]),
                2 => img_bool.extend([true, false]),
                3 => img_bool.extend([true, true]),
                _ => panic!("Please extend the matching arm!")
            }
        }
    }

    img_bool
}

#[test]
fn test_rice_code_fpga() {
    // Most simple case
    let arr = ndarray::array![[0, 0], [0, 0]];
    assert_eq!(
        rice_code(&arr, 1, 2),
        vec![false, false, false, false, false, false, false, false, false, false, false, false]
    );

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

    // Simple case with two parallel units
    let arr = ndarray::array![[0, 255], [0, 254]];
    assert_eq!(
        rice_code(&arr, 2, 2),
        vec![
            false, false, false,
            false, false, false,
            false, true, false,
            true, false, false, false,
        ]
    );

    // Edge case with one pixel
    let arr = ndarray::array![[255]];
    assert_eq!(
        rice_code(&arr, 1, 2),
        vec![false, true, false]
    );
}