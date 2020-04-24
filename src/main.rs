use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use clap::{App, Arg};
use rand::{thread_rng, Rng};

static GIPHY_API_KEY: &str = "API KEY";

mod giphy;

/// Read the history file
fn read_history() -> HashSet<String> {
    let mut history = HashSet::<String>::new();

    let home_dir = dirs::home_dir();
    if home_dir.is_none() {
        return history;
    }

    let mut path = home_dir.unwrap();
    path.push(".giphy_history");

    let file = File::open(path);
    if !file.is_ok() {
        return history;
    }

    let reader = BufReader::new(file.unwrap());
    for line in reader.lines() {
        match line {
            Ok(id) => {
                if !id.is_empty() {
                    history.insert(id);
                }
            },
            Err(_) => {},
        }
    }

    history
}

/// Write to the history file
fn write_history(history: &HashSet<String>) {
    let home_dir = dirs::home_dir();
    if home_dir.is_none() {
        return;
    }

    let mut path = home_dir.unwrap();
    path.push(".giphy_history");

    let file = File::create(path);
    if !file.is_ok() {
        return;
    }

    let mut file = file.unwrap();
    for id in history {
        let mut id = id.to_owned();
        id.push_str("\n");
        match file.write_all(id.as_bytes()) {
            Ok(_) => {},
            Err(_) => {}, // we don't care if it fails
        }
    }
}

fn main() {
    let matches = App::new("Giphy")
        .version("0.4.0")
        .author("Michael Enger <michaelenger@live.com>")
        .about("Searches giphy.com for an appropriate gif")
        .arg(
            Arg::with_name("api_key")
                .short("k")
                .long("apikey")
                .value_name("KEY")
                .help("API key for communicating with Giphy")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("rating")
                .short("r")
                .long("rating")
                .value_name("RATING")
                .help("Rating of the gifs to retrieve")
                .takes_value(true)
                .possible_values(&["g", "pg", "pg-13", "r"])
                .default_value("g")
                .required(true),
        )
        .arg(
            Arg::with_name("markdown")
                .short("m")
                .long("markdown")
                .help("Wraps the resulting URL in some markdown"),
        )
        .arg(
            Arg::with_name("query")
                .help("Text to use when searching for a gif")
                .index(1),
        )
        .get_matches();

    let api_key = if matches.value_of("api_key").is_some() {
        matches.value_of("api_key").unwrap()
    } else {
        GIPHY_API_KEY
    };

    let rating = String::from(matches.value_of("rating").unwrap());
    let show_markdown = matches.is_present("markdown");

    let mut history = read_history();

    let result = match matches.value_of("query") {
        Some(query) => giphy::search(&api_key, &query, &rating),
        None => giphy::trending(&api_key, &rating),
    };

    let mut gifs = match result {
        Err(e) => panic!("Failed to retrieve gifs: {}", e),
        Ok(giphys) => (giphys),
    };

    if gifs.len() < 1 {
        panic!("Giphy did not return enough results");
    }

    let index: usize = thread_rng().gen_range(0, gifs.len());

    let giphy = gifs.swap_remove(index);

    let image = match giphy.images.get("original") {
        Some(image) => image,
        _ => panic!("Unable to extract original image"),
    };

    let url = match &image.url {
        Some(image) => image,
        _ => panic!("Unable to get image URL"),
    };

    history.insert(giphy.id);
    write_history(&history);

    if show_markdown {
        print!("![R+]({})", url);
    } else {
        print!("{}", url);
    }
}
