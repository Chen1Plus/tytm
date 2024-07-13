use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};

mod fsx;
mod manifest;
mod source;

use manifest::Registry;

#[derive(Parser)]
#[command(
    name = "TyTM",
    author = "Chen1Plus",
    about = "Typora Theme Manager",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new theme
    Add { theme: String },

    /// Remove a theme
    #[command(alias = "rm")]
    Remove { theme: String },

    /// List all installed themes
    #[command(alias = "ls")]
    List,
}

fn main() {
    let registry = Registry::from_file("source.json").unwrap();

    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            dbg!("[todo] list");
        }

        Commands::Add { theme } => {
            registry.get_theme(theme).unwrap().install().unwrap();
        }

        Commands::Remove { theme } => {
            dbg!("[todo] remove {}", theme);
        }
    }
}
