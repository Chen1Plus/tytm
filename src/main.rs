use clap::{Parser, Subcommand};
use reqwest::Url;

mod cmds;
mod env;
mod fsx;
mod pkg;

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
    env::init();

    let cli = Cli::parse();
    match cli.command {
        Commands::Update => {
            todo!()
        }

        Commands::Add { url } => {
            use cmds::add::UrlType;
            cmds::add::entry(url, UrlType::Zip)?;
        }

        Commands::Remove { theme, sub } => {
            todo!()
        }

        Commands::List => {
            todo!()
        }
    }

    Ok(())
}
