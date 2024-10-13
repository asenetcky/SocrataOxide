use crate::cli::*;
use crate::data::*;
use crate::opendataurl::*;

use anyhow::Result;
use polars::prelude::*;
use polars_io::ipc::IpcWriter;
use std::fs::File;

pub fn run(args: Args) -> Result<()> {
    let _api_key = args.api_key;
    let _username = args.username;
    let _password = args.password;

    let open_data = OpenDataUrl::new(&args.dataset_url, args.limit, args.offset);
    let output = OutFile::new(args.out_file);
    let data = Data::new(&open_data?);

    match output.out_type {
        OutType::Arrow => {
            let filename = output.file_name.unwrap().to_string();
            let mut file = File::create(filename).expect("could not create file");
            IpcWriter::new(&mut file).finish(&mut data.unwrap().df)?;
        }
        OutType::Csv => {
            let filename = output.file_name.unwrap().to_string();
            let mut file = File::create(filename).expect("could not create file");
            CsvWriter::new(&mut file).finish(&mut data.unwrap().df)?;
        }
        OutType::Json => {
            let filename = output.file_name.unwrap().to_string();
            let mut file = File::create(filename).expect("could not create file");
            JsonWriter::new(&mut file).finish(&mut data.unwrap().df)?;
        }
        OutType::Stdout => {
            println!("{:?}", data.unwrap().df);
        }
    }
    Ok(())
}
