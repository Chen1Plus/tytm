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
    let avail = PkgList::from_file("source.json").unwrap();

    let mut installed =
        InstalledPkgList::from_file(fsx::THEME_DIR.join("pkgs.json")).unwrap_or_default();

    let cli = Cli::parse();
    match &cli.command {
        Commands::List => {
            dbg!("[todo] list");
        }

        Commands::Add { theme } => {
            let pkg = avail.get_pkg(&theme).unwrap();
            installed.add_pkg(pkg.install().unwrap());
            installed.save_to(fsx::THEME_DIR.join("pkgs.json")).unwrap();
        }

        Commands::Remove { theme } => {
            let pkg = installed.get_pkg(&theme).unwrap();
            pkg.uninstall().unwrap();
            installed.rm_pkg(&theme);
            installed.save_to(fsx::THEME_DIR.join("pkgs.json")).unwrap();
        }
    }
}
