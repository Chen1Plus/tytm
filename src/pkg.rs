use std::{
    fs::{self, File},
    path::PathBuf,
    rc::Rc,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json as json;

use crate::fsx;

mod zip;

pub(crate) fn update_manifest() -> Result<()> {
    let tmp_dir = fsx::TempDir::new()?;
    git2::Repository::clone("https://github.com/Chen1Plus/tytm", &tmp_dir)?;
    if !fsx::MANIFEST_DIR.exists() {
        fs::create_dir_all(&*fsx::MANIFEST_DIR)?;
    }
    if !fsx::THEME_DIR.join("tytm-pkgs").exists() {
        fs::create_dir(fsx::THEME_DIR.join("tytm-pkgs"))?;
    }
    fsx::move_dir(tmp_dir.path().join("manifest"), &*fsx::MANIFEST_DIR)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Package {
    pub(crate) id: String,
    name: String,
    version: String,
    source: Rc<dyn Source>,
}

impl Package {
    pub(crate) fn get(id: String) -> Result<Self> {
        json::from_reader(File::open(fsx::MANIFEST_DIR.join(id + ".json"))?).map_err(Into::into)
    }

    pub(crate) fn install(&self) -> Result<()> {
        json::to_writer(
            File::create(
                fsx::THEME_DIR
                    .join("tytm-pkgs")
                    .join(self.id.clone() + ".json"),
            )?,
            &(InstalledPackage {
                id: self.id.clone(),
                name: self.name.clone(),
                version: self.version.clone(),
                added_files: self.source.install()?,
            }),
        )
        .map_err(Into::into)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct InstalledPackage {
    pub(crate) id: String,
    name: String,
    version: String,
    added_files: Vec<PathBuf>,
}

impl InstalledPackage {
    pub(crate) fn get(id: String) -> Result<Self> {
        json::from_reader(File::open(
            fsx::THEME_DIR.join("tytm-pkgs").join(id + ".json"),
        )?)
        .map_err(Into::into)
    }

    pub(crate) fn uninstall(&self) -> Result<()> {
        for file in &self.added_files {
            debug_assert!(file.exists() && file.is_file());
            fs::remove_file(file)?;
        }
        fs::remove_file(
            fsx::THEME_DIR
                .join("tytm-pkgs")
                .join(self.id.clone() + ".json"),
        )?;
        Ok(())
    }
}

#[typetag::serde(tag = "type", content = "value")]
trait Source {
    fn install(&self) -> Result<Vec<PathBuf>>;
}
