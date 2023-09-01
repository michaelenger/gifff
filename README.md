# gifff

Get the URL for a random gif from based on a search term:

```shell
gifff "betty white dab"
gifff "sadness"
gifff "🦑"
```

## Usage

Pass in your search query to get a URL to a random appropriate gif:

```shell
gifff "something funny"
```

Alternatively you can be basic and just get a random trending gif:

```shell
gifff
```

**PROTIP**: Pipe that 🔥 straight to `pbcopy` and paste into your favourite ~colleague harassment~ intra-office communication tool:

```shell
gifff "why the hell would you commit that??" | pbcopy
```

The utility keeps track of which gifs have already been seen in a file in the home directory called `.gifff_history` so that you won't face the embarassment of pasting the same reaction twice.

### Options

* `--ignore-history` Do not filter out previously seen gifs
* `--clear-history` Clear the existing history of gifs
* `--help` Show help text
* `--markdown` Wrap the URL in some markdown - useful for saying 👍 to pull requests
* `--version` Prints version information

## Development

You will need an API key from [Giphy](https://developers.giphy.com/) which you put into the `GIPHY_API_KEY` variable.

```rust
static GIPHY_API_KEY: &str = "YOUR API KEY";
```

### Requirements

* Rust 1.38
