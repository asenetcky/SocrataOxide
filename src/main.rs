use crate::cli::*;
use crate::run::*;
use clap::Parser;

pub mod cli;
pub mod data;
pub mod opendataurl;
pub mod run;

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
