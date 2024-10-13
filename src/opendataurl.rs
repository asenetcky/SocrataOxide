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
        let mut limit = limit_flag;
        let mut offset = offset_flag;

        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "$limit" => limit = value.parse().ok().or(limit),
                "$offset" => offset = value.parse().ok().or(offset),
                _ => {} // Ignore other parameters
            }
        }

        Ok(OpenDataUrl { url, limit, offset })
    }

    pub fn with_params(&self) -> Url {
        let mut url = self.url.clone();
        let mut pairs = url.query_pairs().into_owned().collect::<Vec<_>>();

        if let Some(limit) = self.limit {
            pairs.push(("$limit".to_string(), limit.to_string()));
        }

        if let Some(offset) = self.offset {
            pairs.push(("$offset".to_string(), offset.to_string()));
        }

        url.query_pairs_mut().clear().extend_pairs(pairs);
        url
    }
}
