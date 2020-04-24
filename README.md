# Giphy

Get the URL for a random gif from [Giphy](https://giphy.com/) based on a search query:

```shell
giphy "old white guy dabbing"
giphy "sadness"
giphy "that one with the pitbull waddling down the stairs"
```

## Usage

Pass in your search query to get a URL to a random appropriate gif:

```shell
giphy "something funny"
```

Alternatively you can be basic and just get a random trending gif:

```shell
giphy
```

PROTIP: Pipe that 🔥 straight to `pbcopy` and paste into your favourite ~colleague harassment~ intra-office communication tool:

```shell
giphy "why the hell would you commit that??" | pbcopy
```

The utility keeps track of which gifs have already been seen in a file in the home directory called `.giphy_history` so that you won't face the embarassment of pasting the same reaction twice.

### Options

* `--apikey <KEY>` Specify the API key
* `--ignore-history` Do not filter out previously seen gifs
* `--help` Show help text
* `--markdown` Wrap the URL in some markdown - useful for saying 👍 to pull requests
* `--rating <RATING>` specify the rating of the gif (based on the [rating list](https://developers.giphy.com/docs/optional-settings/#rating))
* `--version` Prints version information

## Development

You will need an API key from [Giphy](https://developers.giphy.com/) which you pass in using the `--apikey` parameter, or put into the `GIPHY_API_KEY` variable.

```rust
static GIPHY_API_KEY: &str = "API KEY";
```

### Requirements

* Rust 1.38

### TODO

* Allow for outputting a different quality URL
