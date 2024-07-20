use std::{
    fs,
    path::{self, Path, PathBuf},
};

use anyhow::Result;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;
use zip::ZipArchive;

use crate::fsx::{self, dirs};

use super::Source;

#[derive(Serialize, Deserialize)]
pub struct Zip {
    url: String,
    content: PathBuf,
    excludes: Vec<PathBuf>,
}

#[typetag::serde]
impl Source for Zip {
    fn save_to(&self, path: &Path) -> Result<()> {
        let tmp_dir = fsx::TempDir::new()?;
        let content_dir = tmp_dir.path().join(&self.content);
        {
            let mut file = fsx::tempfile()?;
            println!("Downloading {}", self.url);
            blocking::get(&self.url)?.copy_to(&mut file)?;
            ZipArchive::new(file)?.extract(&tmp_dir)?;
        }

        for path in self.excludes.iter().map(|p| content_dir.join(p)) {
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else if path.is_file() {
                fs::remove_file(path)?;
            }
        }

        fsx::move_dir(content_dir, path).map_err(Into::into)
    }
}
