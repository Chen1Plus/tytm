use std::{
    fs,
    path::{self, Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::fsx::{self, dirs};

use super::Source;

#[derive(Serialize, Deserialize)]
pub struct Git {
    url: String,
    content: PathBuf,
    excludes: Vec<PathBuf>,
}

#[typetag::serde]
impl Source for Git {
    fn install(&self) -> Result<Vec<PathBuf>> {
        let tmp_dir = fsx::TempDir::new()?;
        let content_dir = path::absolute(tmp_dir.path().join(&self.content))?;

        println!("Cloning {}", self.url);
        git2::Repository::clone(&self.url, &tmp_dir)?;
        debug_assert!(content_dir.exists());
        println!("Installing ...");

        for path in self
            .excludes
            .iter()
            .map(|p| path::absolute(content_dir.join(p)))
        {
            let path = path?;
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else if path.is_file() {
                fs::remove_file(path)?;
            }
        }

        let paths = WalkDir::new(&content_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_path_buf())
            .collect();

        fsx::move_dir(content_dir, dirs::TYPORA_THEME.as_path())?;
        println!("Done");
        Ok(paths)
    }

    fn save_to(&self, path: &Path) -> Result<()> {
        let tmp_dir = fsx::TempDir::new()?;
        let content_dir = tmp_dir.path().join(&self.content);

        println!("Cloning {}", self.url);
        git2::Repository::clone(&self.url, &tmp_dir)?;

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
