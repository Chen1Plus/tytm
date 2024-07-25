use std::{fs, path::Path};

use anyhow::Result;
use relative_path::RelativePathBuf;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use crate::fsx;

#[typetag::serde(tag = "type", content = "value")]
pub(super) trait Source {
    fn save_to(&self, path: &Path) -> Result<()>;
}

#[derive(Serialize, Deserialize)]
pub struct Zip {
    url: String,
    content: RelativePathBuf,
    excludes: Vec<RelativePathBuf>,
}

#[typetag::serde]
impl Source for Zip {
    fn save_to(&self, path: &Path) -> Result<()> {
        let tmp_dir = fsx::TempDir::new()?;
        let content_dir = self.content.to_logical_path(&tmp_dir);
        {
            let mut file = fsx::tempfile()?;
            println!("Downloading {}", self.url);
            blocking::get(&self.url)?.copy_to(&mut file)?;
            ZipArchive::new(file)?.extract(&tmp_dir)?;
        }

        for path in self
            .excludes
            .iter()
            .map(|p| p.to_logical_path(&content_dir))
        {
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else if path.is_file() {
                fs::remove_file(path)?;
            }
        }

        fsx::move_dir(content_dir, path).map_err(Into::into)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Git {
    url: String,
    content: RelativePathBuf,
    excludes: Vec<RelativePathBuf>,
}

#[typetag::serde]
impl Source for Git {
    fn save_to(&self, path: &Path) -> Result<()> {
        let tmp_dir = fsx::TempDir::new()?;
        // let content_dir = tmp_dir.path().join(&self.content);
        let content_dir = self.content.to_logical_path(&tmp_dir);

        println!("Cloning {}", self.url);
        git2::Repository::clone(&self.url, &tmp_dir)?;

        for path in self
            .excludes
            .iter()
            .map(|p| p.to_logical_path(&content_dir))
        {
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else if path.is_file() {
                fs::remove_file(path)?;
            }
        }

        fsx::move_dir(content_dir, path).map_err(Into::into)
    }
}
