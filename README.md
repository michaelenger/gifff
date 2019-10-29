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

You will need an API key from [Giphy](https://developers.giphy.com/).

Then just run it 🤙

```shell
cargo run -- --apikey "api key goes here" "search query goes here"
```

## TODO

* Add reading a config file from `~/.giphy` directory
* Fetch latest featured gifs if no query paramater is passed
* Keep a cache of previously used gifs and avoid duplicates
* Allow for outputting a different
* Add `--rating` CLI param for selecting the rating
