use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use gumdrop::Options;
use rand::{thread_rng, Rng};

static VERSION_NUMBER: &str = "1.3.0";
static HISTORY_FILE: &str = ".gifff_history";

mod giphy;

#[derive(Debug, Options)]
struct CliOptions {
    #[options(free, help = "Text to use when searching for a gif")]
    query: Option<String>,

    #[options(help_flag)]
    help: bool,

    #[options(help = "Wraps the resulting URL in some markdown")]
    markdown: bool,

    #[options(help = "Do not filter out gifs that have already been seen")]
    ignore_history: bool,

    #[options(help = "Clear the existing history")]
    clear_history: bool,

    #[options(help = "Prints version information")]
    version: bool,
}

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
    let opts = CliOptions::parse_args_default_or_exit();

    if opts.version {
        println!("gifff {}", VERSION_NUMBER);
        return Ok(());
    }

    if opts.clear_history {
        write_history(&HashSet::<String>::new()); // kinda dirty solution, but 🤷‍♀️
    }

    let mut history = read_history();

    let api_key = match env::var("GIPHY_API_KEY") {
        Ok(val) => val,
        Err(_) => panic!("Missing GIPHY_API_KEY"),
    };

    let mut gifs = match opts.query {
        Some(query) => giphy::search(api_key.as_str(), &query),
        None => giphy::trending(api_key.as_str()),
    }?;

    if !opts.ignore_history {
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

    let index: usize = thread_rng().gen_range(0..gifs.len());
    let gif = gifs.swap_remove(index);

    if !opts.ignore_history {
        history.insert(gif.id);
        write_history(&history);
    }

    if opts.markdown {
        print!("![R+]({})", gif.url);
    } else {
        print!("{}", gif.url);
    }

    Ok(())
}
