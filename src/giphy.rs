use std::collections::HashMap;
use std::error::Error;
use std::fmt;

use reqwest::StatusCode;
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
    pub id: String,
    pub images: HashMap<String, Image>,
}

#[derive(Debug)]
/// Custom error
struct GiphyError {
    message: String,
}

impl Error for GiphyError {}

impl fmt::Display for GiphyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Deserialize, Debug)]
struct GiphyResponse {
    data: Vec<Giphy>,
}

#[derive(Deserialize, Debug)]
struct GiphyErrorResponse {
    message: String,
}

/// Make a GET request
fn get(url: Url) -> Result<Vec<Giphy>, Box<dyn Error>> {
    let mut response = reqwest::get(url)?;

    match response.status() {
        StatusCode::OK => {
            let body: GiphyResponse = response.json()?;
            Ok(body.data)
        }
        _ => {
            let body: GiphyErrorResponse = response.json()?;
            Err(Box::new(GiphyError {
                message: body.message,
            }))
        }
    }
}

/// Search Giphy for gifs specified by the query
pub fn search(api_key: &str, query: &str, rating: &str) -> Result<Vec<Giphy>, Box<dyn Error>> {
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

    get(url)
}

/// Get recent trending Giphys
pub fn trending(api_key: &str, rating: &str) -> Result<Vec<Giphy>, Box<dyn Error>> {
    let url = Url::parse_with_params(
        "https://api.giphy.com/v1/gifs/trending",
        &[("api_key", api_key), ("limit", "420"), ("rating", rating)],
    )
    .unwrap();

    get(url)
}
