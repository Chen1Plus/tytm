use clap::{Parser, Subcommand};

mod fsx;
// mod manifest;
mod pkg;

// use manifest::{InstalledPkgList, PkgList};
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
    // let avail = PkgList::from_file("source.json").unwrap();

    // let mut installed =
    //     InstalledPkgList::from_file(fsx::THEME_DIR.join("pkgs.json")).unwrap_or_default();

    let cli = Cli::parse();
    match cli.command {
        Commands::Update => {
            pkg::update_manifest().unwrap();
        }

        Commands::Add { theme } => {
            Package::get(theme).unwrap().install().unwrap();
        }

        Commands::Remove { theme } => {
            InstalledPackage::get(theme).unwrap().uninstall().unwrap();
        }

        Commands::List => {
            dbg!("[todo] list");
        }
    }
}
