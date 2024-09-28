use anyhow::Result;
use polars::prelude::*;
use reqwest::{blocking, Error, Response};
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
    // pub fn grab_data(input: Input) -> Result<()> {}

    pub fn new(url: Url, file_type: FileType) -> Self {
        // grab_data(url, file_type)?;
        Self {
            url,
            file_type,
            response,
        }
    }
}

// pub fn build_output(url: &str) -> Output {
//     let url = parse_url(url);
//     let file_type = parse_filetype(url.as_str());

//     let response =
//        match file_type {
//            FileType::Json => {
//            //     let response_json: serde_json::Value = get(url).unwrap().json().unwrap();
//            //     let json = serde_json::to_string(&response_json).unwrap();
//            //     let cursor = Cursor::new(json);
//            //     JsonReader::new(cursor).finish().unwrap()
//            println!("JSON");
//                       let response = blocking::get(url)?;
//                       let json: serde_json::Value = response.json()?;
//                       let json_str = serde_json::to_string(&json)?;
//                       let cursor = std::io::Cursor::new(json_str);
//                       let df = polars::prelude::JsonReader::new(cursor).finish()?;
//                       println!("{}", df);}
//            FileType::Csv => {
//                blocking::get(url.as_str()).expect("cannot get url")
//                println!("CSV");
//                let response = blocking::get(url).unwrap();
//                let csv = response.text().unwrap();
//                let cursor = std::io::Cursor::new(csv);
//                let df = polars::prelude::CsvReader::new(cursor).finish()?;

//            }
//            FileType::UnknownMimeType => {
//                println!("UnknownMimeType");
//            }
//          };

//     Output {
//         url,
//         file_type,
//         response,
//     }
// }

// pub fn build_output(url: &str) -> Result<Output, anyhow::Error> {
//     let url = Url::parse(url)?;

//     let response = blocking::get(url.as_str())?;
//     let file_type = match response.headers().get(CONTENT_TYPE) {
//         Some(content_type) => {
//             let content_type = content_type.to_str()?;
//             if content_type.contains("application/json") {
//                 FileType::Json(content_type.to_string())
//             } else if content_type.contains("text/csv") {
//                 FileType::Csv(content_type.to_string())
//             } else {
//                 FileType::UnknownMimeType
//             }
//         }
//         None => FileType::UnknownMimeType,
//     };
//     Ok(Output {
//         url,
//         file_type,
//         response,
//     })
// }

pub fn grab_data(url: &str) -> Result<()> {
    let file_type = url.split('.').last().unwrap();

    match file_type {
        "json" => {
            println!("JSON");
            let response = blocking::get(url)?;
            let json: serde_json::Value = response.json()?;
            let json_str = serde_json::to_string(&json)?;
            let cursor = std::io::Cursor::new(json_str);
            let df = polars::prelude::JsonReader::new(cursor).finish()?;
            println!("{}", df);
        }
        "csv" => {
            println!("CSV");
            let response = blocking::get(url)?;
            let csv = response.text()?;
            let cursor = std::io::Cursor::new(csv);
            let df = polars::prelude::CsvReader::new(cursor).finish()?;
            println!("{}", df);
        }
        _ => {
            println!("Unknown file type");
        }
    }

    // let url = "https://data.ct.gov/resource/qhtt-czu2.json";
    // let response_json: serde_json::Value = reqwest::blocking::get(url)?.json()?;
    // let json = serde_json::to_string(&response_json)?;
    // let cursor = std::io::Cursor::new(json);
    // let df = polars::prelude::JsonReader::new(cursor).finish()?;
    // println!("{}", df);
    Ok(())
}
