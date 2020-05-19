use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use clap::{App, Arg};
use rand::{thread_rng, Rng};

static HISTORY_FILE: &str = ".gifff_history";

mod gfycat;

/// Read the history file
fn read_history() -> HashSet<String> {
    let mut history = HashSet::<String>::new();

    let home_dir = dirs::home_dir();
    if home_dir.is_none() {
        return history;
    }

    let mut path = home_dir.unwrap();
    path.push(HISTORY_FILE);

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
            }
            Err(_) => {}
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
    path.push(HISTORY_FILE);

    let file = File::create(path);
    if !file.is_ok() {
        return;
    }

    let mut file = file.unwrap();
    for id in history {
        let mut id = id.to_owned();
        id.push_str("\n");
        match file.write_all(id.as_bytes()) {
            Ok(_) => {}
            Err(_) => {} // we don't care if it fails
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("gifff")
        .version("1.1.0")
        .author("Michael Enger <michaelenger@live.com>")
        .about("Searches the web for an appropriate gif")
        .arg(
            Arg::with_name("markdown")
                .short("m")
                .long("markdown")
                .help("Wraps the resulting URL in some markdown"),
        )
        .arg(
            Arg::with_name("ignore_history")
                .short("i")
                .long("ignore-history")
                .help("Do not filter out gifs that have already been seen"),
        )
        .arg(
            Arg::with_name("clear_history")
                .short("c")
                .long("clear-history")
                .help("Clear the existing history"),
        )
        .arg(
            Arg::with_name("query")
                .help("Text to use when searching for a gif")
                .index(1),
        )
        .get_matches();

    let show_markdown = matches.is_present("markdown");
    let ignore_history = matches.is_present("ignore_history");

    if matches.is_present("clear_history") {
        write_history(&HashSet::<String>::new()); // kinda dirty solution, but 🤷‍♀️
    }

    let mut history = read_history();

    let mut gifs = match matches.value_of("query") {
        Some(query) => gfycat::search(&query),
        None => gfycat::trending(),
    }?;

    if !ignore_history {
        // TODO replace with drain_filter when/if available.
        // Ref: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain_filter
        let mut i = 0;
        while i != gifs.len() {
            if history.contains(&gifs[i].id) {
                gifs.remove(i);
            } else {
                i += 1;
            }
        }
    }

    if gifs.len() < 1 {
        panic!("Could not find any gifs");
    }

    let index: usize = thread_rng().gen_range(0, gifs.len());
    let gif = gifs.swap_remove(index);

    if !ignore_history {
        history.insert(gif.id);
        write_history(&history);
    }

    if show_markdown {
        print!("![R+]({})", gif.url);
    } else {
        print!("{}", gif.url);
    }

    Ok(())
}
