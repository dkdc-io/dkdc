mod commands;
mod config;
mod terminal;

use clap::{Parser, Subcommand};
use commands::{config_it, open_it, print_config};

#[derive(Parser)]
#[command(name = "dkdc", about = "Develop knowledge, develop code", color = clap::ColorChoice::Auto, disable_help_subcommand=true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Opens a thing (or alias)", alias = "o")]
    Open { thing: Option<String> },
    #[command(about = "Configures things (and aliases)", alias = "c")]
    Config {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Open { thing } => match thing {
            Some(thing) => open_it(&thing),
            None => print_config(),
        },
        Commands::Config {} => {
            config_it();
        }
    }
}
