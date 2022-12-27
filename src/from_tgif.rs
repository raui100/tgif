use std::time::Instant;

use log::{debug, info, trace};
use rayon::prelude::*;

use crate::args::FromTGIF;
use crate::constants::{CHUNK_SIZE, REV_RICE_INDEX, U8_TO_ARRAY_BOOL};
use crate::header::{Header, STARTING_INDEX};

pub fn run(args: &FromTGIF) {
    info!("Converting {} to {}", args.src, args.dst);

    debug!("Reading the TGIF file from disk");
    let tgif = std::fs::read(&args.src)
        .unwrap_or_else(|_| panic!("Failed reading {}", &args.src));

    debug!("Parsing the header");
    let header = Header::from_u8(&tgif);

    let time = Instant::now();
    debug!("Decoding the TGIF image");
    let img = decode(&tgif[STARTING_INDEX..], &header);

    // Speed in Megabyte / s
    let rate =  1.0 / time.elapsed().as_secs_f64();

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

fn decode(comp: &[u8], header: &Header) -> Vec<u8> {
    let time = Instant::now();
    // Chunks must be dividable into bytes
    assert_eq!(CHUNK_SIZE % 8, 0);
    let mut rice_ind = comp.par_chunks(CHUNK_SIZE / 8)
        .flat_map(|chunk| {
            let mut res: Vec<u8> = Vec::with_capacity(400_000);
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
                    if !code_unary && remainder_index == header.rem_bits {
                        res.push(num);
                        num = 0;
                        remainder_index = 0;
                        code_unary = true;
                    }
                }
            }
            res
        })
        .collect::<Vec<u8>>();
    trace!("Time for decompression: {:?}", time.elapsed());

    let time = Instant::now();
    rice_ind.par_chunks_exact_mut(header.width as usize)
        .for_each(|chunk| {
            let mut prev = 0u8;
            for ind in chunk {
                let delta = REV_RICE_INDEX[*ind as usize];
                prev = prev.wrapping_sub(delta);
                *ind = prev
            }
        });
    trace!("Time for reverse rice index and delta: {:?}", time.elapsed());


    rice_ind
}