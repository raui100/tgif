use std::io::Write;

use clap::Parser;
use log::{debug, LevelFilter};

mod args;
mod encode;
mod decode;

fn main() {
    // TODO: Move this to CLI args
    let parallel_units = 1;
    let remainder_bits = 2;
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
    match (args.src.extension(), args.dst.extension()) {
        (Some("png"), Some("tgif")) => encode::encode(&args, parallel_units,remainder_bits),
        (Some("tgif"), Some("png")) => decode::decode(&args),
        (src, dst) => {
            let src = src.unwrap();
            let dst = dst.unwrap();
            panic!("Converting {src} to {dst} is not supported");
        }
    };
}
