use clap::Parser;

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

    /// Number of bits used to encode the remainder
    #[clap(long, short)]
    pub rem_bits: Option<u8>,
}
impl Cli {
    pub fn verify_arguments(self) -> Operation {
        match (&self.src.extension(), &self.dst.extension()) {
            (Some("tgif"), Some(x)) if x != &"tgif" => {
                if self.rem_bits.is_none() {
                    Operation::FromTGIF(FromTGIF {src: self.src, dst: self.dst})
                } else {
                    panic!("The number of remainder bits can only be set when converting to TGIF")
                }

            },
            (Some(x), Some("tgif")) if x != &"tgif" => {
                if let Some(rem_bits) = self.rem_bits {
                    Operation::ToTGIF(ToTGIF {src: self.src, dst: self.dst, rem_bits})
                } else {
                    panic!("The number of remainder bits must be set when converting to TGIF")
                }
            },
            _ => panic!("Only converting to/from TGIF is supported")
        }
    }
}

pub enum Operation {
    ToTGIF(ToTGIF),
    FromTGIF(FromTGIF)
}

pub struct ToTGIF {
    pub src: camino::Utf8PathBuf,
    pub dst: camino::Utf8PathBuf,
    pub rem_bits: u8,  // Number of Bits that are used for the remainder
}

pub struct FromTGIF {
    pub src: camino::Utf8PathBuf,
    pub dst: camino::Utf8PathBuf,
}
