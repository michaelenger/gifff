use std::{error::Error, fmt};

use reqwest::blocking::Client;
use reqwest::{StatusCode, Url};
use serde::Deserialize;

/// Error from Giphy
#[derive(Debug, Deserialize)]
struct GiphyError {
    status: i32,
    message: String,
}

impl Error for GiphyError {}
impl fmt::Display for GiphyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.status)
    }
}

/// An image response from the API
#[derive(Debug, Deserialize)]
struct GiphyResponse {
    data: Vec<GifObject>,
    meta: MetaObject,
}

#[derive(Debug, Deserialize)]
struct MetaObject {
    status: i32,
    msg: String,
}

#[derive(Debug, Deserialize)]
struct GifObject {
    id: String,
    images: GifObjectImage,
}

#[derive(Debug, Deserialize)]
struct GifObjectImage {
    downsized_large: GifObjectImageSource,
}

#[derive(Debug, Deserialize)]
struct GifObjectImageSource {
    url: String,
}

/// A gif image
#[derive(Debug)]
pub struct GifImage {
    pub id: String,
    pub url: String,
}

fn make_request(url: Url) -> Result<Vec<GifImage>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send()?;

    match response.status() {
        StatusCode::OK => {
            let body: GiphyResponse = response.json()?;
            let images: Vec<GifImage> = body
                .data
                .iter()
                .map(|x| GifImage {
                    id: x.id.clone(),
                    url: x.images.downsized_large.url.clone(),
                })
                .collect();
            Ok(images)
        }
        _ => {
            let body: GiphyResponse = response.json()?;
            let error = GiphyError {
                status: body.meta.status,
                message: body.meta.msg,
            };
            Err(Box::new(error))
        }
    }
}

/// Search for gifs specified by the query
pub fn search(api_key: &str, query: &str) -> Result<Vec<GifImage>, Box<dyn Error>> {
    let url = Url::parse_with_params(
        "https://api.giphy.com/v1/gifs/trending",
        &[("api_key", api_key), ("limit", "50"), ("q", query)],
    )?;

    make_request(url)
}

/// Get recent trending gifs
pub fn trending(api_key: &str) -> Result<Vec<GifImage>, Box<dyn Error>> {
    let url = Url::parse_with_params(
        "https://api.giphy.com/v1/gifs/trending",
        &[("api_key", api_key), ("limit", "50")],
    )?;

    make_request(url)
}
