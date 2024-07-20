use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::fsx;

use super::Source;

#[derive(Serialize, Deserialize)]
pub struct Git {
    url: String,
    content: PathBuf,
    excludes: Vec<PathBuf>,
}

#[typetag::serde]
impl Source for Git {
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
