use anyhow::{Ok, Result};
use clap::Parser;
// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Write};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// My Rust version of RSocrata
pub struct Args {
    /// URL
    #[arg(value_name = "URL")]
    dataset_url: String,

    /// Api Key
    #[arg(short = 'k', long = "key", default_value = "-", value_name = "API_KEY")]
    api_key: String,

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
}

pub fn run(args: Args) -> Result<()> {
    println!("{:?}", args);

    let url = args.dataset_url;
    let _api_key = args.api_key;
    let _username = args.username;
    let _password = args.password;

    // need to grab the file type from the url
    // or looke for a better way to grab MIME type
    // focus on jsons and .csv for now

    let _file_type = url.split('.').last().unwrap();

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
