# phrack-downloader

A command-line tool for managing and downloading Phrack magazine issues.

## Development Environment Setup

1. **Install Rust**
   - If you don't have Rust, install it from [rustup.rs](https://rustup.rs/):
     ```sh
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
2. **Clone the repository**
   ```sh
   git clone https://github.com/hasanen/phrack-downloader.git
   cd phrack-downloader
   ```
3. **Build the project**
   ```sh
   cargo build --release
   ```
4. **Run the tool**
   ```sh
   cargo run -- --help
   ```

## How to Use the Tool

### List All Configurations

To list all available configuration keys:

```sh
cargo run -- config
```

### Get a Single Config Value

To get the value of a specific configuration key:

```sh
cargo run -- config <config_key>
```

Example:

```sh
cargo run -- config download-path
```

### Set a New Config Value

To set a configuration value:

```sh
cargo run -- config <config_key> <value>
```

Example:

```sh
cargo run -- config download-path ./downloads/
```

### Commands TBD

```sh
cargo run -- download-issue --issue 1 # download single issue
cargo run -- download-issue --issue 1 --refresh # with refresh, old downloaded issue will be removed
cargo run -- download-issue --all-issues # downloads all non-downloaded issues
cargo run -- download-issue --all-issues --refresh # purges existing downloads and re-downloads all issues
cargo run -- check-new-issues # Check if there are new issues
cargo run -- convert-issue --issue 1 --format txt # generates single .txt of all articles
cargo run -- convert-issue --all-issues --format txt # generates single .txt of all articles per publication/issue
cargo run -- sync-with-calibre # ability to sync generated txt, pdf and epub files with calibre library, with proper metadata (eg. using series-field)
```
