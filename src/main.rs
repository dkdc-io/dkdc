mod commands;
mod config;
mod error;
mod terminal;

use clap::{Parser, Subcommand};
use commands::{config_it, gif_it, open_it, print_config};

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
    #[command(about = "Convert a video to a gif", alias = "g")]
    Gif {
        input: String,
        output: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Open { thing } => match thing {
            Some(thing) => open_it(&thing),
            None => {
                print_config();
                Ok(())
            }
        },
        Commands::Config {} => config_it(),
        Commands::Gif { input, output } => gif_it(input, output),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
