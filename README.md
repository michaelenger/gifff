# Giphy

Get the URL for a random gif from [Giphy](https://giphy.com/) based on a search query:

```shell
giphy "old white guy dabbing"
```

## Usage

You will need an API key from [Giphy](https://developers.giphy.com/):

```shell
giphy --apikey "api key goes here" "search query goes here"
```

PROTIP: Pipe that 🔥 straight to `pbcopy` and paste into your favourite ~colleague harassment~ intra-office communication tool:

```shell
giphy "why the hell would you commit that??" | pbcopy
```

Alternatively you can be basic and just get a random trending gif:

```shell
giphy
```

### Options

* `--apikey <KEY>` specify the API key
* `--help` Show help text
* `--markdown` Wrap the URL in some markdown useful for saying 👍 to pull requests
* `--number <AMOUNT>` how many gifs to retrieve
* `--rating <RATING>` specify the rating of the gif (based on the [rating list](https://developers.giphy.com/docs/optional-settings/#rating))
* `--version` Prints version information

## Development

### Requirements

* Rust 1.38

### TODO

* Add reading a config file from `~/.giphy` directory
* Keep a cache of previously used gifs and avoid duplicates
* Allow for outputting a different quality URL
