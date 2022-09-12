use ::clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "TGIF")]
#[clap(about = "Encodes and decodes grayscale images from/into the Turbo Gray Image Format")]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Input image (eg: TGIF, PNG, ...)
    #[clap(value_parser)]
    pub src: camino::Utf8PathBuf,

    /// Output image (eg: TGIF, PNG, ...)
    #[clap(value_parser)]
    pub dst: camino::Utf8PathBuf,

    /// Number of bits used to encode the remainder
    #[clap(long, short)]
    pub remainder_bits: Option<u8>,

    /// Number of encoding units used to encode the image in parallel
    #[clap(long, short)]
    pub parallel_encoding_units: Option<u32>,

    /// Adds no header to the TGIF file
    #[clap(long, action)]
    pub no_header: bool,

    /// Image width
    #[clap(long, short)]
    pub width: Option<u32>,

    /// Image height
    #[clap(long, short)]
    pub height: Option<u32>,
}
