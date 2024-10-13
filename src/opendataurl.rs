use anyhow::Result;
use url::Url;

// we'll keep these as optional
// we'll have default behavior for these
// be: grab everything normally
// and then maybe for limit default_missing_value be
// the usual odp standard 1000
// we'll grab the params from the url or the flags
// url takes priority

#[derive(Debug)]
pub enum FileType {
    Json,
    Csv,
    UnknownMimeType,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct OpenDataUrl {
    pub url: Url,
    limit: Option<u32>,
    offset: Option<u32>,
    pub file_type: FileType,
}

#[allow(dead_code)]
impl OpenDataUrl {
    pub fn new(url: &str, limit_flag: Option<u32>, offset_flag: Option<u32>) -> Result<Self> {
        let mut url = Url::parse(url)?;
        let limit = limit_flag;
        let offset = offset_flag;

        let file_type = match url.path().split('.').last() {
            Some("json") => FileType::Json,
            Some("csv") => FileType::Csv,
            _ => FileType::UnknownMimeType,
        };

        let binding = url.clone();
        let mut pairs = binding.query_pairs().collect::<Vec<_>>();

        if let Some(limit_flag) = limit_flag {
            pairs.push(("$limit".to_string().into(), limit_flag.to_string().into()));
        }

        if let Some(offset_flag) = offset_flag {
            pairs.push(("$offset".to_string().into(), offset_flag.to_string().into()));
        }

        url.query_pairs_mut().clear().extend_pairs(pairs);

        Ok(OpenDataUrl {
            url,
            limit,
            offset,
            file_type,
        })
    }
}
