# phrack-downloader

A command-line tool for managing and downloading Phrack magazine issues.

## How to Contribute

Contributions are welcome! To contribute:

1. Fork the repository and create your branch from `main`.
2. Make your changes and add tests if applicable.
3. Ensure the project builds and passes all checks.
4. Use [Conventional Commits](#commit-message-guidelines) for your commit messages.
5. Open a pull request describing your changes.

Please read the [Commit Message Guidelines](#commit-message-guidelines) section for details on writing commit messages.

### Development Environment Setup

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

### Download Issues

Download a single issue:

```sh
cargo run -- download-issue --issue 1
```

Download a single issue and refresh (remove old downloaded issue first):

```sh
cargo run -- download-issue --issue 1 --refresh
```

Download all non-downloaded issues:

```sh
cargo run -- download-issue --all-issues
```

Purge existing downloads and re-download all issues:

```sh
cargo run -- download-issue --all-issues --refresh
```

### Commands TBD

```sh
cargo run -- export-issue --issue 1 --format txt --output-path ./path # generates single .txt of all articles
cargo run -- export-issue --all-issues --format txt --output-path ./path # generates single .txt of all articles per publication/issue
cargo run -- sync-with-calibre # ability to sync generated txt, pdf and epub files with calibre library, with proper metadata (eg. using series-field)
```

## Commit Message Guidelines

This project uses the [Conventional Commits](https://www.conventionalcommits.org/) specification for commit messages. Please use this format when making commits. Example types include `feat`, `fix`, `docs`, `chore`, and more. This helps automate changelogs and maintain a clear project history.

Example:

```
feat: add config subcommand
fix(config): handle missing config file error
docs: update README with usage instructions
refactor: simplify download logic
test: add tests for config parser
chore: update dependencies
```

For more details, see the [Conventional Commits documentation](https://www.conventionalcommits.org/).

## TODO:

- [ ] Implement commands in "Commands TBD"
- [ ] Add GH Actions to check the build
- [ ] Add GH action to publish build to crates.io
