use clap::{Parser, Subcommand};

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
        config_key: ConfigKey,
        value: Option<String>,
    },
}

fn main() {
    let args = CliArgs::parse();
    let mut config = load_config();

    dbg!(&args);

    match &args.command {
        Commands::Config { config_key, value } => {
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
