# Giphy

Get a random gif from [Giphy](https://giphy.com/) based on a search query:

```shell
giphy "old white guy dabbing"
```

Pipe that 🔥 straight to `pbcopy` and paste into your favourite ~colleague harassment~ intra-office communication tool:

```shell
giphy "why the hell would you commit that??" | pbcopy
```

## Requirements

* Rust 1.38

## Usage

Copy (and then modify) the config file:

```shell
cp config.example.toml config.toml
```

Run it 🤙

```shell
cargo run -- "your query goes here"
```

## TODO

* Send in API Key with a CLI paramater
* Fetch latest featured gifs if no query paramater is passed
* Keep a cache of previously used gifs and avoid duplicates
* Allow for outputting a different
