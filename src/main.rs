use clap::{App, Arg};
use rand::{thread_rng, Rng};

static GIPHY_API_KEY: &str = "API KEY";

mod giphy;

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

    if show_markdown {
        print!("![R+]({})", url);
    } else {
        print!("{}", url);
    }
}
