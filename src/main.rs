use clap::{App, Arg};
use rand::{thread_rng, Rng};

mod giphy;

/// Check whether a string is a positive number
fn is_positive_number(val: String) -> Result<(), String> {
    match val.parse::<usize>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("must be a positive number"))
    }
}

fn main() {
    let matches = App::new("Giphy")
        .version("0.3.0")
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
                .help("Rating of the gifs to retrieve")
                .takes_value(true)
                .possible_values(&["g", "pg", "pg-13", "r"])
                .default_value("g")
                .required(true),
        )
        .arg(
            Arg::with_name("amount")
                .short("n")
                .long("amount")
                .value_name("AMOUNT")
                .help("Amount of gifs to retreive")
                .takes_value(true)
                .default_value("1")
                .required(true)
                .validator(is_positive_number),
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

    let api_key = String::from(matches.value_of("api_key").unwrap());
    let rating = String::from(matches.value_of("rating").unwrap());
    let amount_of_gifs: usize = matches.value_of("amount").unwrap().parse().unwrap();
    let show_markdown = matches.is_present("markdown");

    let result = match matches.value_of("query") {
        Some(query) => giphy::search(&api_key, &query, &rating),
        None => giphy::trending(&api_key, &rating),
    };

    let mut gifs = match result {
        Err(e) => panic!("Failed to retrieve gifs: {}", e),
        Ok(giphys) => (giphys),
    };

    if gifs.len() < amount_of_gifs {
        panic!("Giphy did not return enough results");
    }

    for i in 0..amount_of_gifs {
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

        if i != 0 {
            print!("\n");  // newline between results only
        }

        if show_markdown {
            print!("![R+]({})", url);
        } else {
            print!("{}", url);
        }
    }
}
