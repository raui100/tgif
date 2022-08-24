use::clap::Parser;
use camino;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Input image (eg: TGIF, PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, AVIF, PNM, DDS, TGA, ...)
    #[clap(long)]
    pub src: camino::Utf8PathBuf,

    /// Output image (eg: TGIF, PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, AVIF, PNM, DDS, TGA, ...)
    #[clap(long)]
    pub dst: camino::Utf8PathBuf,

    /// Adds no header to the TGIF file
    #[clap(long, action)]
    pub no_header: bool


}