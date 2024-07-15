use std::{
    fs::{self, File},
    path::PathBuf,
    rc::Rc,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json as json;

use crate::fsx::{self, dirs};

mod zip;

pub(crate) fn update_manifest() -> Result<()> {
    let tmp_dir = fsx::TempDir::new()?;
    git2::Repository::clone("https://github.com/Chen1Plus/tytm", &tmp_dir)?;
    fsx::move_dir(tmp_dir.path().join("manifest"), &*dirs::TYTM_MANIFEST)?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Package {
    id: String,
    name: String,
    version: String,
    source: Rc<dyn Source>,
}

impl Package {
    pub(crate) fn get(id: String) -> Result<Self> {
        json::from_reader(File::open(dirs::TYTM_MANIFEST.join(id + ".json"))?).map_err(Into::into)
    }

    pub(crate) fn install(&self) -> Result<()> {
        json::to_writer(
            File::create(dirs::TYPORA_MANIFEST.join(self.id.clone() + ".json"))?,
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
    id: String,
    name: String,
    version: String,
    added_files: Vec<PathBuf>,
}

impl InstalledPackage {
    pub(crate) fn get(id: String) -> Result<Self> {
        json::from_reader(File::open(dirs::TYPORA_MANIFEST.join(id + ".json"))?).map_err(Into::into)
    }

    pub(crate) fn uninstall(&self) -> Result<()> {
        for file in &self.added_files {
            debug_assert!(file.exists() && file.is_file());
            fs::remove_file(file)?;
        }
        fs::remove_file(dirs::TYPORA_MANIFEST.join(self.id.clone() + ".json"))?;
        Ok(())
    }
}

#[typetag::serde(tag = "type", content = "value")]
trait Source {
    fn install(&self) -> Result<Vec<PathBuf>>;
}
