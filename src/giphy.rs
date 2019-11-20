use std::collections::HashMap;

use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
/// An image
pub struct Image {
    pub frames: Option<String>,
    pub hash: Option<String>,
    pub height: Option<String>,
    pub mp4: Option<String>,
    pub mp4_size: Option<String>,
    pub size: Option<String>,
    pub url: Option<String>,
    pub webp: Option<String>,
    pub webp_size: Option<String>,
    pub width: Option<String>,
}

#[derive(Deserialize, Debug)]
/// A Giphy gif
pub struct Giphy {
    pub images: HashMap<String, Image>,
}

#[derive(Deserialize, Debug)]
struct GiphySearchResponse {
    data: Vec<Giphy>,
}

/// Search Giphy for gifs specified by the query
pub fn search(api_key: &str, query: &str, rating: &str) -> Result<Vec<Giphy>, reqwest::Error> {
    let url = Url::parse_with_params(
        "https://api.giphy.com/v1/gifs/search",
        &[
            ("api_key", api_key),
            ("q", query),
            ("limit", "420"),
            ("rating", rating),
        ],
    )
    .unwrap();

    let body: GiphySearchResponse = reqwest::get(url)?.json()?;

    return Ok(body.data);
}
