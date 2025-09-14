// SPDX-License-Identifier: MIT

//! A command-line tool for managing and downloading Phrack magazine issues.

#![deny(missing_docs)]
use clap::{ArgGroup, Parser, Subcommand};
use comfy_table::Table;
use enum_iterator::all;

mod config;
mod downloader;
mod strict_string;
use crate::config::{ConfigKey, load_config, save_config};
use crate::downloader::Downloader;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    /// Name of the person to greet
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Config {
        #[arg(value_enum)]
        config_key: Option<ConfigKey>,
        value: Option<String>,
    },
    DownloadIssue(DownloadIssueArgs),
}
#[derive(Parser, Debug)]
#[command(group(
    ArgGroup::new("download_issue")
        .required(true)
        .args(&["issue", "all_issues"])
))]
struct DownloadIssueArgs {
    #[arg(long)]
    issue: Option<u32>,

    #[arg(long = "all-issues", default_value_t = false)]
    all_issues: bool,
}

fn main() {
    let args = CliArgs::parse();
    let mut config = load_config();

    match &args.command {
        Commands::Config { config_key, value } => {
            let mut table = Table::new();
            table.set_header(vec!["Config Key", "Value"]);

            if config_key.is_none() {
                for key in all::<ConfigKey>().collect::<Vec<_>>() {
                    table.add_row(vec![format!("{}", key.as_arg()), config.get_value(&key)]);
                }
            } else {
                let config_key = config_key.as_ref().unwrap();
                if value.is_some() {
                    config.set_value(config_key, value.as_ref().unwrap());
                    save_config(&config);
                    println!("Updated config");
                }

                table.add_row(vec![
                    format!("{}", config_key.as_arg()),
                    config.get_value(&config_key),
                ]);
            }

            println!("{table}");
        }
        Commands::DownloadIssue(args) => {
            let downloader = Downloader::new(config);

            if args.all_issues {
                downloader.download_all_issues();
            } else if let Some(issue) = args.issue {
                downloader.download_issue(issue);
            }
        }
    }
}
