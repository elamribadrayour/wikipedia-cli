# Wikipedia CLI

Wikipedia CLI is a command-line tool that allows you to interact with Wikipedia, providing functionalities like searching for pages, retrieving page content, images, categories, links, languages, and view statistics.

## Features

- **Search**: Find Wikipedia pages based on a search query.
- **Content**: Retrieve the full content of a Wikipedia page.
- **Images**: List all images associated with a Wikipedia page.
- **Categories**: Get categories that a Wikipedia page belongs to.
- **Links**: List all internal links on a Wikipedia page.
- **Languages**: Retrieve available language versions for a Wikipedia page.
- **Views**: Get view statistics for a Wikipedia page over a specific period.

## Installation

To install the Wikipedia CLI tool, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/elamribadrayour/wikipedia-cli.git
cd wikipedia-cli
cargo build --release
```

This will create an executable in the `target/release` directory.

## Usage

The tool is built using [Clap](https://docs.rs/clap), which allows for easy interaction through subcommands and arguments. Below are usage examples for each command:

### Search

```bash
wikipedia-cli search --query "Rust"
```

### Get Page Content

```bash
wikipedia-cli content --query "Rust"
```

### Get Images

```bash
wikipedia-cli images --query "Rust"
```

### Get Categories

```bash
wikipedia-cli categories --query "Rust"
```

### Get Links

```bash
wikipedia-cli links --query "Rust"
```

### Get Languages

```bash
wikipedia-cli languages --query "Rust"
```

### Get Views

```bash
wikipedia-cli views --query "Rust" --start-date "20230101" --nb-days 30
```

## Commands

- **Search**: Searches Wikipedia for pages related to the given query.
- **Content**: Retrieves the full text content of the specified Wikipedia page.
- **Images**: Lists all images found on the specified Wikipedia page.
- **Categories**: Lists all categories that the specified Wikipedia page belongs to.
- **Links**: Lists all hyperlinks found on the specified Wikipedia page.
- **Languages**: Lists all language versions available for the specified Wikipedia page.
- **Views**: Retrieves view statistics for the specified Wikipedia page over a given period, starting from the specified date.

## Dependencies

- **Tokio**: Asynchronous runtime for the Rust programming language.
- **Clap**: Command-line arguments parser.
- **Serde JSON**: Serialization and deserialization library.
- **Wikipedia Client**: Custom library for interacting with Wikipedia's API.
- **Chrono**: Date and time library for Rust.

## Contributing

Contributions are welcome. Please feel free to submit a pull request or open an issue on GitHub.

## License

This project is licensed under the MIT License. See the [license](license) file for more details.
