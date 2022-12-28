use std::time::Instant;

use log::{debug, info, trace};
use rayon::prelude::*;

use crate::args::FromTGIF;
use crate::constants::{CHUNK_SIZE, REV_RICE_INDEX, U8_TO_ARRAY_BOOL};
use crate::header::{Header, STARTING_INDEX};

pub fn run(args: &FromTGIF) {
    info!("Converting {} to {}", args.src, args.dst);

    debug!("Reading the TGIF file from disk");
    let tgif = std::fs::read(&args.src).unwrap_or_else(|_| panic!("Failed reading {}", &args.src));

    debug!("Parsing the header");
    let header = Header::from_u8(&tgif);

    let time = Instant::now();
    debug!("Decoding the TGIF image");
    let img = decode(&tgif[STARTING_INDEX..], &header);

    // Speed in Megabyte / s
    let rate = 1.0 / time.elapsed().as_secs_f64();

    debug!("Saving the original image to disk");
    image::save_buffer(
        &args.dst,
        &img,
        header.width,
        header.height,
        image::ColorType::L8,
    )
    .unwrap();

    info!("Finished! Decoding speed was {rate:.3} FPS")
}

/// Count's the numbers of consecutive "1" and adds them to a Vec<u8>
///
/// # Data
/// The data looks similar to "1101110" and contains numbers in unary notation which means:
/// 0 <-> "0", 1 <-> "10", 2 <-> "110", 3 <-> "1110", etc
fn decode_without_remainder(chunk: &[u8], res: &mut Vec<u8>) {
    let mut unary = 0u8;
    for num in chunk {
        for bit in U8_TO_ARRAY_BOOL[*num as usize] {
            if bit {
                unary += 1
            } else {
                res.push(unary);
                unary = 0
            }
        }
    }
}

/// count's the numbers of consecutive "1", parses the remainder and adds them to a Vec<u8>
///
/// # Data
/// The data looks similar to "110001.." and contains a number in unary notation and a remainder.
/// The remainder contains of `rem_bits` bits. For "11001" and `rem_bits=3` we would have:
/// "110" <-> 2 (unary notation) and "001" <-> 1 (MSB)
/// The resulting number is (2 << 3) + 1 = 17
///
/// # Algorithm
/// A number always consists of the unary part with unknown number of bits and the remainder with
/// `rem_bits` bits. Bit-padding is only used at end of chunks and only with "1"s.
/// This results in the "edge" case that a chunk ends with "..1111". So there can be a unary without
/// a remainder.
/// However there is never the case that a chunk ends with "..1101" and `rem_bits=2`. Remainders are
/// always complete.
fn decode_with_remainder(chunk: &[u8], res: &mut Vec<u8>, rem_bits: u8) {
    // Iterating over the bits of the image
    let mut it = chunk.iter().flat_map(|n| U8_TO_ARRAY_BOOL[*n as usize]);

    loop {
        // Determining the number of consecutive "1"
        let mut unary = 0;
        while let Some(true) = it.next() {
            unary += 1;
        }
        // Checking if there is a remainder.
        if let Some(bit) = it.next() {
            let mut remainder = bit as u8;
            for _ in 1..rem_bits {
                // If there is a remainder, it is always complete
                let bit = it.next().unwrap() as u8;
                remainder = (remainder << 1) + bit;
            }
            res.push((unary << rem_bits) + remainder);
        } else {
            break;
        }
    }
}

fn decode(comp: &[u8], header: &Header) -> Vec<u8> {
    let time = Instant::now();
    // Chunks must be dividable into bytes
    assert_eq!(CHUNK_SIZE % 8, 0);
    let mut rice_ind = comp
        .par_chunks(CHUNK_SIZE / 8)
        .flat_map(|chunk| {
            // Doesn't reallocate in the case of 50 % compression rate
            let mut res: Vec<u8> = Vec::with_capacity(CHUNK_SIZE / 2);

            if header.rem_bits == 0 {
                decode_without_remainder(chunk, &mut res);
            } else {
                decode_with_remainder(chunk, &mut res, header.rem_bits);
            }
            res
        })
        .collect::<Vec<u8>>();
    trace!("Time for decompression: {:?}", time.elapsed());

    let time = Instant::now();

    rice_ind
        .par_chunks_exact_mut(header.width as usize)
        .for_each(|chunk| {
            let mut prev = 0u8;
            for ind in chunk {
                let delta = REV_RICE_INDEX[*ind as usize]; // rice-index -> delta
                prev = prev.wrapping_sub(delta); // delta -> pixel
                *ind = prev
            }
        });
    trace!(
        "Time for reverse rice index and delta: {:?}",
        time.elapsed()
    );

    rice_ind
}
