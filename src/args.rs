use clap::Parser;
use log::{debug, info};

#[derive(Parser, Debug)]
#[clap(name = "TGIF")]
#[clap(about = "Encodes and decodes grayscale images from/into the Turbo Gray Image Format")]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Input image (eg: TGIF, PNG, ...)
    #[clap(value_parser)]
    pub src: camino::Utf8PathBuf,

    /// Output image (eg: TGIF, PNG, ...)
    #[clap(value_parser)]
    pub dst: camino::Utf8PathBuf,

    /// Number of bits used to encode the remainder. Should be 0..=7. [Default: 2]
    #[clap(short, long)]
    pub rem_bits: Option<u8>,

    /// Size of self contained chunk in Kibibyte. Should be equal to L1 cache size. [Default: 128]
    #[clap(short, long)]
    pub chunk_size: Option<u32>,
}

impl Cli {
    pub fn verify_arguments(self) -> Operation {
        match (&self.src.extension(), &self.dst.extension()) {
            (Some("tgif"), Some(x)) if x != &"tgif" => {
                if self.rem_bits.is_some() || self.chunk_size.is_some() {
                    info!("The provided CLI arguments are not being used when decoding TGIF")
                }
                Operation::FromTGIF(FromTGIF {
                    src: self.src,
                    dst: self.dst,
                })
            }

            (Some(x), Some("tgif")) if x != &"tgif" => {
                let rem_bits = match self.rem_bits {
                    Some(rem_bits) => rem_bits,
                    None => {
                        debug!("Using default value for rem_bits: 2");
                        2
                    }
                };

                let chunk_size = match self.chunk_size {
                    Some(chunk_size) => chunk_size,
                    None => {
                        debug!("Using default value for chunk_size: 129");
                        128
                    }
                };

                assert!(
                    rem_bits < 8,
                    "The number of remainder bits should be lower than 8"
                );
                assert_ne!(chunk_size, 0, "The chunk size must be higher than 0");

                Operation::ToTGIF(ToTGIF {
                    src: self.src,
                    dst: self.dst,
                    rem_bits,
                    chunk_size: chunk_size * 1024 * 8, // Converting to Kibibyte
                })
            }
            _ => panic!("Only converting to/from TGIF is supported"),
        }
    }
}

#[derive(Debug)]
pub enum Operation {
    ToTGIF(ToTGIF),
    FromTGIF(FromTGIF),
}

#[derive(Debug)]
pub struct ToTGIF {
    /// Path to source file (eg: PNG or BMP)
    pub src: camino::Utf8PathBuf,
    /// Path to TGIF destination file
    pub dst: camino::Utf8PathBuf,
    /// Number of bits that are used for the remainder
    pub rem_bits: u8,
    /// Number of Kibibytes that are used for the self contained chunk
    pub chunk_size: u32,
}

#[derive(Debug)]
pub struct FromTGIF {
    /// Path to source file (eg: PNG or BMP)
    pub src: camino::Utf8PathBuf,
    /// Path to TGIF destination file
    pub dst: camino::Utf8PathBuf,
}
