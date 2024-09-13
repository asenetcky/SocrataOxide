use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// My Rust version of `uniq`
struct Args {
    /// Input file
    #[arg(default_value = "-", value_name = "IN_FILE")]
    in_file: String,

    /// Output file
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// prefix line by number of occurrences
    #[arg(short, long)]
    count: bool,
}

fn run(args: Args) -> () {
    println!("{:?}", args);
}

// fn run(args: Args) -> Result<()> {
//     let mut file = open(&args.in_file).map_err(|e| anyhow!("{}: {e}", args.in_file))?;
//
//     let mut out_file: Box<dyn Write> = match &args.out_file {
//         Some(out_name) => Box::new(File::create(out_name)?),
//         _ => Box::new(io::stdout()),
//     };
//
//     let mut print = |num: u64, text: &str| -> Result<()> {
//         if num > 0 {
//             if args.count {
//                 write!(out_file, "{num:>4} {text}")?;
//             } else {
//                 write!(out_file, "{text}")?;
//             }
//         };
//         Ok(())
//     };
//
//     let mut line = String::new();
//     let mut previous = String::new();
//     let mut count: u64 = 0;
//
//     loop {
//         let bytes = file.read_line(&mut line)?;
//         if bytes == 0 {
//             break;
//         }
//
//         if line.trim_end() != previous.trim_end() {
//             print(count, &previous)?;
//             previous = line.clone();
//             count = 0;
//         }
//
//         count += 1;
//         line.clear();
//     }
//
//     print(count, &previous)?;
//
//     Ok(())
// }
//
// fn open(filename: &str) -> Result<Box<dyn BufRead>> {
//     match filename {
//         "-" => Ok(Box::new(BufReader::new(io::stdin()))),
//         _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
//     }
// }

