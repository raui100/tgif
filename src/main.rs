use clap::Parser;
use std::io::Write;

use crate::args::Operation;
use log::{debug, LevelFilter};

mod args;
mod constants;
mod from_tgif;
mod header;
mod to_tgif;

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
    let args: Operation = args::Cli::parse().verify_arguments();
    debug!("Arguments: {:?}", &args);

    match &args {
        Operation::ToTGIF(args) => to_tgif::run(args),
        Operation::FromTGIF(args) => from_tgif::run(args),
    }
}
