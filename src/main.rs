use std::fs::File;
use std::io::prelude::*;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    api_key: String,
}

fn read_config(path: &str) -> Result<Config, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}

fn main() {
    println!("Hello, world!");
    match read_config("config.toml") {
        Err(e) => {
            println!("Error! Unable to read config.toml file: {}", e);
        },
        Ok(config) => { dbg!(config.api_key); },
    }
}
