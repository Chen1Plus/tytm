use std::{fs, path::PathBuf, rc::Rc};

use anyhow::Result;
use serde::{Deserialize, Serialize};

mod zip;

#[derive(Serialize, Deserialize)]
pub(crate) struct Package {
    pub(crate) id: String,
    name: String,
    version: String,
    source: Rc<dyn Source>,
}

impl Package {
    pub(crate) fn install(&self) -> Result<InstalledPackage> {
        Ok(InstalledPackage {
            id: self.id.clone(),
            name: self.name.clone(),
            version: self.version.clone(),
            added_files: self.source.install()?,
        })
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
    pub(crate) fn uninstall(&self) -> Result<()> {
        for file in &self.added_files {
            debug_assert!(file.exists() && file.is_file());
            fs::remove_file(file)?;
        }
        Ok(())
    }
}

#[typetag::serde(tag = "type", content = "value")]
trait Source {
    fn install(&self) -> Result<Vec<PathBuf>>;
}
