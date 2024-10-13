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
struct OpenDataUrl {
    url: Url,
    limit: Option<u32>,
    offset: Option<u32>,
}

// impl OpenDataUrl {
//     fn new(limit: LimitMode, offset: u32) -> Self {}
// }

// // since not everyone wants to escape all the bash expansion
// // characters, I need to add clap flags for this
// // Extract query parameters
// let mut url_limit = None;
// let mut url_offset = None;

// // this is a target for an enum if there are more query parameters
// for (key, value) in url.query_pairs() {
//     match key.as_ref() {
//         "$limit" => url_limit = value.parse().ok(),
//         "$offset" => url_offset = value.parse().ok(),
//         _ => {} // Ignore other parameters
//     }
// }

// let pagination = Pagination {
//     limit: url_limit.unwrap_or(10), // Default limit
//     offset: url_offset.unwrap_or(0),
// };
