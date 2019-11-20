use clap::{App, Arg};
use rand::{thread_rng, Rng};

mod giphy;

fn main() {
    let matches = App::new("Giphy")
        .version("0.1.0")
        .author("Michael Enger <michaelenger@live.com>")
        .about("Searches giphy.com for an appropriate gif")
        .arg(
            Arg::with_name("api_key")
                .short("k")
                .long("apikey")
                .value_name("KEY")
                .help("API key for communicating with Giphy")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("rating")
                .short("r")
                .long("rating")
                .value_name("RATING")
                .help("API key for communicating with Giphy")
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
                .required(true)
                .index(1),
        )
        .get_matches();

    let query = matches.value_of("query").unwrap();
    let api_key = matches.value_of("api_key").unwrap();
    let rating = matches.value_of("rating").unwrap();
    let show_markdown = matches.is_present("markdown");

    let results = match giphy::search(&api_key, &query, &rating) {
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

    if show_markdown {
        print!("![R+]({})", url);
    } else {
        print!("{}", url);
    }
}
