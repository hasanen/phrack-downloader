// SPDX-License-Identifier: MIT

use clap::{Parser, Subcommand};
use enum_iterator::all;

mod config;
mod strict_string;
use crate::config::{ConfigKey, load_config};

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
}

fn main() {
    let args = CliArgs::parse();
    let mut config = load_config();

    dbg!(&args);

    match &args.command {
        Commands::Config { config_key, value } => {
            if config_key.is_none() {
                println!("Current configs:");
                for key in all::<ConfigKey>().collect::<Vec<_>>() {
                    println!("- {:?} -> {}", key, config.get_value(&key));
                }
                return;
            }

            let config_key = config_key.as_ref().unwrap();
            if value.is_some() {
                println!(
                    "Setting config key: {:?} to value: {}",
                    config_key,
                    value.as_ref().unwrap()
                );
                config.set_value(config_key, value.as_ref().unwrap());
            }

            println!(
                "config: {:?} -> {}",
                config_key,
                config.get_value(&config_key)
            );
        }
    }
}
