use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use clap::{Arg, App};
use rand::{thread_rng, Rng};
use reqwest::Url;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    api_key: String,
}

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

fn read_config(path: &str) -> Result<Config, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents)?;

    Ok(config)
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
    let config: Config = match read_config("config.toml") {
        Err(e) => panic!("Unable to read config.toml file: {}", e),
        Ok(config) => config,
    };

    let matches = App::new("Giphy")
        .version("0.1.0")
        .author("Michael Enger <michaelenger@live.com>")
        .about("Searches giphy.com for an appropriate gif")
        .arg(
            Arg::with_name("query")
                .help("Text to use when searching for a gif")
                .required(true)
                .index(1),
        )
        .get_matches();

    let query = matches.value_of("query").unwrap();

    let results = match make_request(&config.api_key, &query) {
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
