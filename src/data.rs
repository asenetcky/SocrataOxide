use anyhow::Result;
use polars::prelude::*;
use reqwest::blocking;
use serde_json::Value;
use std::io::Cursor;
use url::Url;

#[derive(Debug)]
pub enum FileType {
    Json,
    Csv,
    UnknownMimeType,
}

#[derive(Debug)]
pub enum OutType {
    Stdout,
    Arrow,
    Json,
    Csv,
}

#[derive(Debug)]
pub struct OutFile {
    pub out_type: OutType,
    pub file_name: Option<String>,
}

#[derive(Debug)]
pub struct Data {
    pub df: DataFrame,
    // url: Url,
    // file_type: FileType,
}

// this doesnt work with urls that have the params in it now
// need to make data new() accept opendataurl struct
// and then it can its magic with file etc...
impl Data {
    pub fn new(url: &Url) -> Result<Self> {
        let file_type = match url.path().split('.').last() {
            Some("json") => FileType::Json,
            Some("csv") => FileType::Csv,
            _ => FileType::UnknownMimeType,
        };

        let df = match file_type {
            FileType::Json => {
                let response = blocking::get(url.as_str())?;
                let json: Value = response.json()?;
                let json_str = serde_json::to_string(&json)?;
                let cursor = Cursor::new(json_str);
                let df = polars::prelude::JsonReader::new(cursor).finish()?;
                df
            }
            FileType::Csv => {
                let response = blocking::get(url.as_str())?;
                let csv = response.text()?;
                let cursor = Cursor::new(csv);
                let df = polars::prelude::CsvReader::new(cursor).finish()?;
                df
            }
            FileType::UnknownMimeType => {
                panic!("UnknownMimeType");
            }
        };

        Ok(Self { df })
    }
}

impl OutFile {
    pub fn new(file_name: Option<String>) -> Self {
        let out_type = match &file_name {
            Some(file_name) => {
                let file_type = match file_name.split('.').last() {
                    Some("arrow") => OutType::Arrow,
                    Some("csv") => OutType::Csv,
                    Some("json") => OutType::Json,
                    _ => panic!("Unknown file type"),
                };
                file_type
            }
            None => OutType::Stdout,
        };

        Self {
            out_type,
            file_name,
        }
    }
}
