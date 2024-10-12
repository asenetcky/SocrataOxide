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
pub enum OutType {
    Stdout,
    Arrow,
    Csv,
}

#[derive(Debug)]
pub struct OutFile {
    pub out_type: OutType,
    pub file_name: Option<String>,
}

#[derive(Debug)]
struct Pagination {
    limit: i32,
    offset: i32,
}

#[derive(Debug)]
pub struct Data {
    pub df: DataFrame,
    url: Url,
    file_type: FileType,
}

impl Data {
    pub fn new(url: &str) -> Result<Self> {
        let url = Url::parse(url)?;
        let file_type = match url.path().split('.').last() {
            Some("json") => FileType::Json,
            Some("csv") => FileType::Csv,
            _ => FileType::UnknownMimeType,
        };

        // since not everyone wants to escape all the bash expansion
        // characters, I need to add flags for this
        // Extract query parameters
        let mut url_limit = None;
        let mut url_offset = None;

        // this is a target for an enum if there are more query parameters
        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "$limit" => url_limit = value.parse().ok(),
                "$offset" => url_offset = value.parse().ok(),
                _ => {} // Ignore other parameters
            }
        }

        let pagination = Pagination {
            limit: url_limit.unwrap_or(10), // Default limit
            offset: url_offset.unwrap_or(0),
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

        Ok(Self { df, url, file_type })
    }
}

impl OutFile {
    pub fn new(file_name: Option<String>) -> Self {
        let out_type = match &file_name {
            Some(file_name) => {
                let file_type = match file_name.split('.').last() {
                    Some("arrow") => OutType::Arrow,
                    Some("csv") => OutType::Csv,
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
