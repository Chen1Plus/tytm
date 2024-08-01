use anyhow::Result;
use relative_path::RelativePathBuf;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use tempfile::{tempdir, tempfile, TempDir};
use zip::ZipArchive;

use crate::fsx::Obj;

#[typetag::serde(tag = "type", content = "value")]
pub(super) trait Source {
    fn download(&self) -> Result<TempDir>;
}

#[derive(Serialize, Deserialize)]
pub struct Zip {
    url: String,
    content: RelativePathBuf,
    excludes: Vec<RelativePathBuf>,
}

#[typetag::serde]
impl Source for Zip {
    fn download(&self) -> Result<TempDir> {
        let tmp_dir = tempdir()?;
        let content_dir = self.content.to_logical_path(&tmp_dir);
        {
            println!("Downloading {}", self.url);
            let mut file = tempfile()?;
            blocking::get(&self.url)?.copy_to(&mut file)?;

            println!("Extracting...");
            ZipArchive::new(file)?.extract(&tmp_dir)?;
        }

        for obj in self
            .excludes
            .iter()
            .map(|p| Obj::from(p.to_logical_path(&content_dir)))
        {
            obj.remove()?;
        }

        Obj::from(content_dir).move_inside_to(&tmp_dir)?;
        Ok(tmp_dir)
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
    fn download(&self) -> Result<TempDir> {
        let tmp_dir = tempdir()?;
        let content_dir = self.content.to_logical_path(&tmp_dir);

        println!("Cloning {}", self.url);
        git2::Repository::clone(&self.url, &tmp_dir)?;

        for obj in self
            .excludes
            .iter()
            .map(|p| Obj::from(p.to_logical_path(&content_dir)))
        {
            obj.remove()?;
        }

        Obj::from(content_dir).move_inside_to(&tmp_dir)?;
        Ok(tmp_dir)
    }
}
