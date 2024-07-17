use std::{
    fs,
    path::{self, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

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

        let paths = fsx::scan_dir(path::absolute(&content_dir)?)?
            .into_iter()
            .map(|p| dirs::TYPORA_THEME.join(p.strip_prefix(&content_dir).unwrap()))
            .collect();

        fsx::move_dir(content_dir, dirs::TYPORA_THEME.as_path())?;
        println!("Done");
        Ok(paths)
    }
}
