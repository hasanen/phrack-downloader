// SPDX-License-Identifier: MIT

//! A command-line tool for managing and downloading Phrack magazine issues.

#![deny(missing_docs)]
use clap::{ArgGroup, Parser, Subcommand};
use comfy_table::Table;
use enum_iterator::all;
use std::process;

mod config;
mod downloader;
mod models;
mod phrack;
mod phrack_issue_manager_error;
mod strict_string;
use crate::config::{ConfigKey, load_config, save_config};
use crate::downloader::Downloader;
use crate::phrack_issue_manager_error::PhrackIssueManagerError;

#[derive(Copy, Clone, Debug)]
enum ExitCode {
    Success = 0,
    GenericError = 1,
}
impl ExitCode {
    fn as_i32(&self) -> i32 {
        *self as i32
    }
}
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

    #[arg(long = "refresh", default_value_t = false)]
    refresh: bool,
}
#[tokio::main]
async fn main() {
    let args = CliArgs::parse();
    let mut config = match load_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Failed to load configuration: {}", e);
            process::exit(ExitCode::GenericError.as_i32());
        }
    };

    match &args.command {
        Commands::Config { config_key, value } => {
            let mut table = Table::new();
            table.set_header(vec!["Config Key", "Value"]);

            if let Some(config_key) = config_key {
                let config_key = config_key;

                match value {
                    Some(v) => {
                        config.set_value(config_key, v);

                        match save_config(&config) {
                            Ok(_) => println!("Updated config"),
                            Err(e) => handle_error(&e),
                        }
                    }
                    None => (),
                }

                table.add_row(vec![
                    format!("{}", config_key.as_arg()),
                    config.get_as_str(&config_key),
                ]);
            } else {
                for key in all::<ConfigKey>().collect::<Vec<_>>() {
                    table.add_row(vec![format!("{}", key.as_arg()), config.get_as_str(&key)]);
                }
            }

            println!("{table}");
        }
        Commands::DownloadIssue(args) => {
            let downloader = Downloader::new(config);
            let refresh = args.refresh;

            if args.all_issues {
                match downloader.download_all_issues(refresh).await {
                    Ok(_) => {}
                    Err(e) => handle_error(&e),
                };
            } else if let Some(issue) = args.issue {
                match downloader.download_issue(&issue.into(), refresh).await {
                    Ok(_) => {}
                    Err(e) => handle_error(&e),
                }
            }
        }
    }

    process::exit(ExitCode::Success.as_i32());
}

fn handle_error(error: &PhrackIssueManagerError) {
    let exit_code = match error {
        _ => ExitCode::GenericError,
    };

    eprintln!("Error: {}", &error);
    process::exit(exit_code.as_i32());
}
