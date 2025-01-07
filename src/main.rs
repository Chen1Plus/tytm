use std::fs;

use clap::{Parser, Subcommand};

mod cmds;
mod fsx;
mod pkg;

use pkg::{InstalledPackage, Manifest};
use reqwest::Url;

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
        /// The url of the theme to add
        url: Url,
    },

    /// Remove a theme
    #[command(alias = "rm")]
    Remove {
        /// The theme to remove
        theme: String,

        /// The sub-packages to remove
        #[arg(short, long)]
        sub: Option<Vec<String>>,
    },

    /// List all installed themes
    #[command(alias = "ls")]
    List,
}

fn main() -> anyhow::Result<()> {
    fsx::defs::init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Update => {
            pkg::Manifest::update()?;
        }

        Commands::Add { url } => {
            use cmds::add::UrlType;
            cmds::add::entry(url, UrlType::Zip)?;
        }

        Commands::Remove { theme, sub } => {
            let mut pkg = InstalledPackage::get(&theme).expect("Theme not installed");
            if let Some(id) = sub {
                for id in &id {
                    pkg.remove_sub(id)?;
                }
                pkg.save()?;
            } else {
                pkg.uninstall()?;
                fs::remove_file(fsx::defs::TYPORA_MANIFEST.join(theme + ".json"))?;
            }
        }

        Commands::List => {
            println!("Installed themes:");
            fs::read_dir(fsx::defs::TYPORA_MANIFEST.as_path())?
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
                .map(|e| e.path().file_stem().unwrap().to_owned())
                .for_each(|e| println!("{}", e.to_str().unwrap()));
        }
    }

    Ok(())
}
