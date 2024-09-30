use anyhow::Result;
use polars::prelude::*;
use reqwest::{blocking, blocking::Response};
use serde_json::Value;
use std::io::Cursor;
use url::{ParseError, Url};

#[derive(Debug)]
pub enum FileType {
    Json,
    Csv,
    UnknownMimeType,
}

#[derive(Debug)]
pub struct Input {
    url: Url,
    file_type: FileType,
}

#[derive(Debug)]
pub struct Output {
    url: Url,
    file_type: FileType,
    response: Response,
}

impl Input {
    pub fn new(url: &str) -> Result<Self> {
        let url = Url::parse(url)?;
        let file_type = match url.as_str().split('.').last() {
            Some("json") => FileType::Json,
            Some("csv") => FileType::Csv,
            _ => FileType::UnknownMimeType,
        };
        Ok(Self { url, file_type })
    }
}

impl Output {
    pub fn new(input: Input) -> Self {
        let url = input.url;
        let file_type = input.file_type;
        let response = grab_response(&url, &file_type).unwrap();
        Self {
            url,
            file_type,
            response,
        }
    }
}

pub fn grab_response(url: &Url, file_type: &FileType) -> Result<Response, anyhow::Error> {
    let response = match file_type {
        FileType::Json | FileType::Csv => {
            let response = blocking::get(url.as_str())?;
            response
        }
        FileType::UnknownMimeType => {
            return Err(anyhow::anyhow!("UnknownMimeType"));
        }
    };
    Ok(response)
}

pub fn grab_data(file_type: &FileType, output_response: &Response) -> Result<polars::prelude::DataFrame, PolarsError> {
    let dataframe = match file_type {
        FileType::Json => {
            let json: Value = output_response.json().unwrap();
            let json_str = serde_json::to_string(&json).unwrap();
            let cursor = Cursor::new(json_str);
            let df = polars::prelude::JsonReader::new(cursor).finish().unwrap();
            df
        }
        FileType::Csv => {
            let csv = output_response.text().unwrap();
            let cursor = Cursor::new(csv);
            let df = polars::prelude::CsvReader::new(cursor).finish().unwrap();
            df
        }
        FileType::UnknownMimeType => {
            panic!("UnknownMimeType");
        }
    }
    Ok(dataframe)
}

//         pub fn grab_data(self: Self) -> Result<(), anyhow::Error> {
//         let url = self.url;
//         let file_type = self.file_type;
//         let response = match file_type {
//             FileType::Json => {
//                 let response = blocking::get(url.as_str())?;
//                 let json: Value = response.json()?;
//                 let json_str = serde_json::to_string(&json)?;
//                 let cursor = Cursor::new(json_str);
//                 let df = polars::prelude::JsonReader::new(cursor).finish()?;
//                 println!("{}", df);
//             }
//             FileType::Csv => {
//                 let response = blocking::get(url.as_str())?;
//                 let csv = response.text()?;
//                 let cursor = Cursor::new(csv);
//                 let df = polars::prelude::CsvReader::new(cursor).finish()?;
//                 println!("{}", df);
//             }
//             FileType::UnknownMimeType => {
//                 println!("UnknownMimeType");
//             }
//         };
//         Ok(())
//     }
// }
