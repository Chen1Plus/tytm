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
    let avail = Registry::from_file("source.json").unwrap();

    let mut installed =
        Registry::from_file(fsx::data_dir().unwrap().join("Typora/themes/pkgs.json"))
            .unwrap_or_default();

    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            dbg!("[todo] list");
        }

        Commands::Add { theme } => {
            let theme = avail.get_theme(theme).unwrap();
            theme.install().unwrap();
            installed.add_theme(theme.clone());
            installed
                .save_to(fsx::data_dir().unwrap().join("Typora/themes/pkgs.json"))
                .unwrap();
        }

        Commands::Remove { theme } => {
            dbg!("[todo] remove {}", theme);
        }
    }
}
