use crate::cli::*;
use crate::data::*;
use crate::opendataurl::*;

use anyhow::Result;
use clap::Parser;
use polars::prelude::*;
use polars_io::ipc::IpcWriter;
use std::fs::File;

pub fn run(args: Args) -> Result<()> {
    let url = args.dataset_url;
    let _api_key = args.api_key;
    let _username = args.username;
    let _password = args.password;
    let limit = args.limit;
    let offset = args.offset;

    let my_url = OpenDataUrl::new(&url, limit, offset);
    println!("{:?}", my_url);

    let output = OutFile::new(args.out_file);
    let mut data = Data::new(&url)?;

    match output.out_type {
        OutType::Arrow => {
            let filename = output.file_name.unwrap().to_string();
            let mut file = File::create(filename).expect("could not create file");
            IpcWriter::new(&mut file).finish(&mut data.df)?;
        }
        OutType::Csv => {
            let filename = output.file_name.unwrap().to_string();
            let mut file = File::create(filename).expect("could not create file");
            CsvWriter::new(&mut file).finish(&mut data.df)?;
        }
        OutType::Json => {
            let filename = output.file_name.unwrap().to_string();
            let mut file = File::create(filename).expect("could not create file");
            JsonWriter::new(&mut file).finish(&mut data.df)?;
        }
        OutType::Stdout => {
            println!("{:?}", data.df);
        }
    }
    Ok(())
}
