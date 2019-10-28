use std::collections::HashMap;

use clap::{Arg, App};
use rand::{thread_rng, Rng};
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Image {
    frames: Option<String>,
    hash: Option<String>,
    height: Option<String>,
    mp4: Option<String>,
    mp4_size: Option<String>,
    size: Option<String>,
    url: Option<String>,
    webp: Option<String>,
    webp_size: Option<String>,
    width: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Giphy {
    images: HashMap<String, Image>,
}

#[derive(Deserialize, Debug)]
struct GiphyResponse {
    data: Vec<Giphy>,
}

fn make_request(api_key: &str, query: &str) -> Result<Vec<Giphy>, reqwest::Error> {
    let url = Url::parse_with_params("https://api.giphy.com/v1/gifs/search", &[
        ("api_key", api_key),
        ("q", query),
        ("limit", "50")
    ]).unwrap();


    let body: GiphyResponse = reqwest::get(url)?
        .json()?;

    return Ok(body.data);
}

fn main() {
    let matches = App::new("Giphy")
        .version("0.1.0")
        .author("Michael Enger <michaelenger@live.com>")
        .about("Searches giphy.com for an appropriate gif")
        .arg(Arg::with_name("api_key")
            .short("k")
            .long("apikey")
            .value_name("key")
            .help("API key for communicating with Giphy")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("query")
            .help("Text to use when searching for a gif")
            .required(true)
            .index(1))
        .get_matches();

    let query = matches.value_of("query").unwrap();
    let api_key = matches.value_of("api_key").unwrap();

    let results = match make_request(&api_key, &query) {
        Err(e) => panic!("Failed to retrieve gifs: {}", e),
        Ok(giphys) => (giphys),
    };

    if results.len() == 0 {
        panic!("Giphy returned 0 results");
    }

    let index: usize = thread_rng().gen_range(0, results.len());

    let image = match results[index].images.get("original") {
        Some(image) => image,
        _ => panic!("Unable to extract original image"),
    };

    let url = match &image.url {
        Some(image) => image,
        _ => panic!("Unable to get image URL"),
    };

    print!("{}", url);
}
