use anyhow::Result;
use url::Url;

// we'll keep these as optional
// we'll have default behavior for these
// be: grab everything normally
// and then maybe for limit default_missing_value be
// the usual odp standard 1000
// we'll grab the params from the url or the flags
// url takes priority

#[allow(dead_code)]
#[derive(Debug)]
pub struct OpenDataUrl {
    url: Url,
    limit: Option<u32>,
    offset: Option<u32>,
}

#[allow(dead_code)]
impl OpenDataUrl {
    pub fn new(url: &str, limit_flag: Option<u32>, offset_flag: Option<u32>) -> Result<Self> {
        let url = Url::parse(url)?;
        let mut limit = None;
        let mut offset = None;

        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "$limit" => limit = value.parse().ok(),
                "$offset" => offset = value.parse().ok(),
                _ => {} // Ignore other parameters
            }
        }

        // prioritize url params over flags
        let limit = limit.or(limit_flag);
        let offset = offset.or(offset_flag);

        Ok(OpenDataUrl { url, limit, offset })
    }
}
