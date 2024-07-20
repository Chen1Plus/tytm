use clap::{Parser, Subcommand};

mod fsx;
mod pkg;

use pkg::{InstalledPackage, Package};
use walkdir::WalkDir;

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
    Add {
        theme: String,
        #[arg(long)]
        sub: Option<Vec<String>>,
    },

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

        Commands::Add { theme, sub } => {
            let pkg = Package::get(theme).expect("Theme not found");
            if let Some(id) = sub {
                pkg.install(&id).unwrap();
            } else {
                pkg.install_default().unwrap();
            }
        }

        Commands::Remove { theme } => {
            // InstalledPackage::get(theme)
            //     .expect("Theme not installed")
            //     .uninstall()
            //     .unwrap();
        }

        Commands::List => {
            println!("Installed themes:");
            WalkDir::new(fsx::dirs::TYPORA_MANIFEST.as_path())
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .map(|e| e.path().file_stem().unwrap().to_owned())
                .for_each(|e| println!("{}", e.to_str().unwrap()));
        }
    }
}
