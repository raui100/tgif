use::clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Input image (eg: TGIF, PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, AVIF, PNM, DDS, TGA, ...)
    #[clap(long)]
    pub src: std::path::PathBuf,

    /// Output image (eg: TGIF, PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, AVIF, PNM, DDS, TGA, ...)
    #[clap(long)]
    pub dst: std::path::PathBuf,

    /// Adds no header to the TGIF file
    #[clap(long, action)]
    pub no_header: bool
}