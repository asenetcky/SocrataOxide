use anyhow::{Ok, Result};
use polars::prelude::*;
use reqwest::header::CONTENT_TYPE;
use reqwest::{blocking, Error, Response, Url};

enum FileType {
    Json(String),
    Csv(String),
    UnknownMimeType,
}

struct Output {
    url: Url,
    file_type: FileType,
    response: Response,
}

// need to put data in a buffer or something so
// if we pull monster data sets it doesnt
// use up all the memory

// pub fn build_output(url: &str) -> Result<Output, reqwest::Error> {
//     let url = Url::parse(url);
//     let response = get(url.as_str())?;
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
