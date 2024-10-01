use crate::data::*;
use anyhow::Result;
use clap::Parser;
use polars::prelude::*;
use polars_io::ipc::IpcWriter;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// My Rust version of RSocrata written as a cli app
pub struct Args {
    /// URL
    #[arg(value_name = "URL")]
    dataset_url: String, // work on vectors later

    /// Api Key
    #[arg(short = 'k', long = "key", default_value = "-", value_name = "API_KEY")]
    api_key: String,

    /// Output File
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// Username
    #[arg(
        short = 'n',
        long = "username",
        default_value = "-",
        value_name = "USERNAME"
    )]
    username: String,

    /// Password
    #[arg(
        short = 'p',
        long = "password",
        default_value = "-",
        value_name = "PASSWORD"
    )]
    password: String,
    // maybe some helpers for the page vs row number attribute in the api
    // ^ in case folks dont want everything by default
}

pub fn run(args: Args) -> Result<()> {
    let url = args.dataset_url;
    let _api_key = args.api_key;
    let _username = args.username;
    let _password = args.password;

    let output = OutFile::new(args.out_file);
    let mut data = Data::new(&url)?;

    match output.out_type {
        OutType::Arrow => {
            let filename = output.file_name.unwrap().to_string();
            let mut file = File::create(filename).expect("could not create file");
            IpcWriter::new(&mut file).finish(&mut data.df)?;
        }
        OutType::Csv => {
            // data.df.write_csv(output.file_name.unwrap())?;
            println!("{:?}", data.df);
        }
        OutType::Stdout => {
            // data.df.write_csv(output.file_name.unwrap())?;
            println!("{:?}", data.df);
        }
    }

    // let mut out_file: Box<dyn Write> = match output.out_type {
    //     OutType::Arrow => Box::new(File::create(output.file_name.unwrap())?),
    //     OutType::Csv => Box::new(File::create(output.file_name.unwrap())?),
    //     OutType::Stdout => Box::new(io::stdout()),
    // };

    // let mut print = |num: u64, text: &str| -> Result<()> {
    //     if num > 0 {
    //         if args.count {
    //             write!(out_file, "{num:>4} {text}")?;
    //         } else {
    //             write!(out_file, "{text}")?;
    //         }
    //     };
    //     Ok(())
    // };

    Ok(())
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
