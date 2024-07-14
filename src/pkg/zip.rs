use std::{
    fs,
    path::{self, PathBuf},
};

use anyhow::Result;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use crate::fsx;

use super::Source;

#[derive(Serialize, Deserialize)]
pub struct Zip {
    url: String,
    content: PathBuf,
    excludes: Vec<PathBuf>,
}

#[typetag::serde]
impl Source for Zip {
    fn install(&self) -> Result<Vec<PathBuf>> {
        let tmp_dir = fsx::TempDir::new()?;
        let tmp_dir = path::absolute(tmp_dir.path())?;
        let content_dir;
        {
            let mut file = fsx::tempfile()?;
            blocking::get(&self.url)?.copy_to(&mut file)?;
            ZipArchive::new(file)?.extract(&tmp_dir)?;

            debug_assert!(self.content.is_relative());
            content_dir = path::absolute(tmp_dir.join(&self.content))?;
            debug_assert!(content_dir.exists());
        }

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
                (*fsx::THEME_DIR.clone())
                    .to_path_buf()
                    .join(p.strip_prefix(&content_dir).unwrap())
            })
            .collect();

        fs::create_dir_all(&*fsx::THEME_DIR)?;
        fsx::move_dir(content_dir, &*fsx::THEME_DIR)?;
        Ok(paths)
    }
}