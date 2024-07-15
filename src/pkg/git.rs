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

        for ex in &self.excludes {
            debug_assert!(ex.is_relative());
            let ex_path = path::absolute(content_dir.join(&ex))?;
            debug_assert!(ex_path.exists());

            if ex_path.is_dir() {
                fs::remove_dir_all(ex_path)?;
            } else if ex_path.is_file() {
                fs::remove_file(ex_path)?;
            }
        }

        let paths = fsx::scan_dir(path::absolute(&content_dir)?)?
            .into_iter()
            .map(|p| {
                dirs::TYPORA_THEME
                    .to_path_buf()
                    .join(p.strip_prefix(&content_dir).unwrap())
            })
            .collect();

        fsx::move_dir(content_dir, &*dirs::TYPORA_THEME)?;
        println!("Done");
        Ok(paths)
    }
}
