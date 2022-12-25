use std::io::Write;

use clap::Parser;
use log::{debug, LevelFilter};

mod args;
mod decode;
mod encode;
mod constants;

fn main() {
    // Setting up the logging environment
    env_logger::Builder::new()
        .format(move |buf, record| {
            writeln!(
                buf,
                "{}:{} | {} | {} | {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .filter(Some("tgif"), LevelFilter::Trace)
        .init();

    debug!("Parsing arguments from CLI");
    let args = args::Args::parse();

    debug!("Checking the arguments for completeness and correctness");
    if args.dst.extension() == Some("tgif") { // Encoding from an image (eg: PNG, BMP, ...) to TGIF
        check_remainder_bits_and_parallel_encoding_units(&args);
    } else { // Encoding TGIF to a graphic format (eg: PNG, BMP, ...)
        if args.no_header {
            check_remainder_bits_and_parallel_encoding_units(&args);
            assert!(args.width.is_some(), "Please specify the image width! (eg: -w 128)");
            assert!(args.height.is_some(), "Please specify the image height (eg: -h 128)");
        }
    }

    // Encoding/Decoding the image
    match (args.src.extension(), args.dst.extension()) {
        (Some(_), Some("tgif")) => encode::encode(&args),
        (Some("tgif"), Some(_)) => decode::decode(&args),
        (src, dst) => {
            let src = src.unwrap();
            let dst = dst.unwrap();
            panic!("Converting {src} to {dst} is not supported");
        }
    };
}

fn check_remainder_bits_and_parallel_encoding_units(args: &args::Args) {
    match args.remainder_bits {
        None => panic!("Please specify the number bits used for the remainder (eg: ... -r 2)"),
        Some(bits) if bits >= 8 => panic!("Please specify less than 8 bits for the remainder (eg: ... -r 2)"),
        _ => (),  // Everything looks fine!
    }

    match args.parallel_encoding_units {
        None => panic!("Please specify the number of parallel encoding units (eg: ... -p 1)"),
        Some(0) => panic!("The number of parallel encoding units must be greater than zero (eg: ... -p 1)"),
        _ => (),  // Everything looks fine
    }
}
