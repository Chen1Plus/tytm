use clap::{Parser, Subcommand};

mod fsx;
mod pkg;

use pkg::{InstalledPackage, Package};

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
    /// Update the manifest
    Update,

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
    fsx::dirs::init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Update => {
            pkg::update_manifest().unwrap();
        }

        Commands::Add { theme } => {
            Package::get(theme)
                .expect("Theme not found")
                .install()
                .unwrap();
        }

        Commands::Remove { theme } => {
            InstalledPackage::get(theme)
                .expect("Theme not installed")
                .uninstall()
                .unwrap();
        }

        Commands::List => {
            dbg!("[todo] list");
        }
    }
}
