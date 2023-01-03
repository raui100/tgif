use log::{debug, info, trace};
use ndarray::Axis;
use nshare::ToNdarray2;
use std::io::Write;

use crate::args;
use crate::constants::{POW_OF_TWO, RICE_INDEX};
use crate::header::Header;

pub fn run(args: &args::ToTGIF) {
    info!("Converting {} to {}", args.src, args.dst);
    debug!("Reading the image from disk and converting it into an 2D ndarray");
    let image = image::open(&args.src)
        .expect("Failed reading input file.")
        .to_luma8() // Coercing into 8-bit grayscale image
        .into_ndarray2();

    debug!("Coding the original image with rice coding");
    let mut img = encode(&image, args.rem_bits, args.chunk_size as usize);

    trace!("Padding the end with '1'");
    img.extend(vec![true; 8 - (image.len() % 8)]);

    trace!("Creating the header of the compressed image");
    let header = Header::new(
        image.shape()[1] as u32,
        image.shape()[0] as u32,
        args.chunk_size,
        args.rem_bits,
    )
    .to_u8();

    trace!("Combining header with the compressed image");
    let img = header
        .into_iter()
        .chain(img.chunks_exact(8).map(|chunk|
                // Creates an u8 from [bool; 8]
                chunk.iter().fold(0u8, |a, b| (a << 1) + *b as u8)))
        .collect::<Vec<u8>>();

    debug!("Writing the TGIF image to disk: {}", args.dst);
    let mut file = std::fs::File::create(&args.dst).expect("Failed creating destination file");
    file.write_all(&img)
        .expect("Failed writing the image to disk");

    let rate = img.len() as f64 / image.len() as f64 * 100.0;
    info!("Finished! Achieved compression rate of {rate:.4} %")
}

fn encode(image: &ndarray::Array2<u8>, rem_bits: u8, chunk_size: usize) -> Vec<bool> {
    assert!(
        rem_bits <= 7,
        "No compression is possible with 8 or more remainder bits"
    );

    // The remainder is smaller than this number remainder < rem_max (∀ remainder)
    let rem_max = 2_u8.pow(rem_bits as u32);

    // Stores the encoded image as a vector of bool
    // Capacity is estimated for no compression to prevent reallocation
    let image_size = image.len() * 8; // Number of bits in the image
    let mut img: Vec<bool> = Vec::with_capacity(image_size);

    // Counter that keeps tracks of how many bits are in the current chunk
    let mut chunk: usize = 0;

    // Counter that keeps track of how many bits are being used on padding
    let mut padding: usize = 0;

    // Iterating over the image
    debug!("Encoding the image as Vec<bool>");
    for axis in image.axis_iter(Axis(0)) {
        let mut prev: u8 = 0; // All pixel outside of the image are defined as 0
        for pixel in axis {
            let delta = prev.wrapping_sub(*pixel); // Calc the delta
            let rice = RICE_INDEX[delta as usize]; // Determines the rice index
            let quotient = rice / rem_max;
            let remainder = rice % rem_max;
            let bits = quotient as usize + 1 + rem_bits as usize;

            // Bit-padding in case this would overstep the predetermined CHUNK_SIZE
            if chunk + bits > chunk_size {
                //
                padding += chunk_size - chunk;
                img.extend(vec![true; chunk_size - chunk]);
                chunk = 0;
            }

            chunk += bits;
            prev = *pixel; // Updating the previous pixel
            unary_coding(&mut img, quotient); // Unary coding of the quotient
            remainder_coding(&mut img, remainder, rem_bits); // Binary coding of the rem
        }
    }

    debug!(
        "Used {:.2} % Bits for padding: {}",
        100.0 * (padding as f64 / image_size as f64),
        padding
    );
    img
}

/// Codes the remainder as boolean binary with `remainder_bits` bit-width
fn remainder_coding(img: &mut Vec<bool>, rem: u8, rem_bits: u8) {
    debug_assert!(rem_bits <= 8); // Hoping for better optimization
    debug_assert!(rem < 2u8.pow(rem_bits as u32));
    img.extend(
        (0..rem_bits)
            .rev() // <-> Most significant bit
            .map(|ind| rem & POW_OF_TWO[ind as usize] != 0),
    )
}

/// Unary coding of the quotient
fn unary_coding(img: &mut Vec<bool>, quot: u8) {
    img.extend(vec![true; quot as usize]);
    img.push(false);
}
