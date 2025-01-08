use std::collections::HashSet;
use std::fs::{self, File};
use std::io;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json as json;
use tempfile::tempdir;

use crate::env;
use crate::fsx::{Obj, ObjName, ShareDir};

mod source;

#[derive(Serialize, Deserialize)]
pub(crate) struct Manifest {
    id: String,
    name: String,
    version: String,
    source: Box<dyn source::Source>,
    assets: HashSet<ObjName>,
    pkgs: HashSet<SubPackage>,
    default: HashSet<String>,
}

impl Manifest {
    pub(crate) fn update() -> Result<()> {
        let tmp_dir = tempdir()?;
        println!("Fetching manifests...");
        git2::Repository::clone("https://github.com/Chen1Plus/tytm", &tmp_dir)?;
        Obj::from(tmp_dir.path().join("manifest")).move_inside_to(env::TYTM_MANIFEST.as_path())?;
        println!("Manifests updated.");
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct SubPackage {
    id: String,
    file: ObjName,
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct InstalledPackage {
    id: String,
    name: String,
    version: String,
    assets: HashSet<Obj>,
    pkgs: HashSet<InstalledSubPackage>,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
struct InstalledSubPackage {
    id: String,
    file: Obj,
}

impl InstalledPackage {
    pub(crate) fn get(id: &str) -> io::Result<Self> {
        Ok(json::from_reader(File::open(
            env::TYPORA_MANIFEST.join(id).with_extension("json"),
        )?)
        .expect("Invalid manifest."))
    }

    pub(crate) fn save(&mut self) -> Result<()> {
        debug_assert!(!self.pkgs.is_empty(), "Try to save an empty package.");

        let path = env::TYPORA_MANIFEST.join(&self.id).with_extension("json");
        if path.exists() {
            fs::remove_file(&path)?;
        }
        json::to_writer(File::create(&path)?, self).map_err(Into::into)
    }

    pub(crate) fn uninstall(&mut self) -> io::Result<()> {
        for path in self.pkgs.iter().map(|p| &p.file) {
            fs::remove_file(path)?;
        }
        self.pkgs.clear();
        self.clear_assets()
    }

    // do nothing if the sub theme not installed
    pub(crate) fn remove_sub(&mut self, id: &str) -> io::Result<()> {
        debug_assert!(self.pkgs.iter().filter(|pkg| pkg.id == id).count() <= 1);
        let Some(pkg) = self.pkgs.iter().find(|pkg| pkg.id == id) else {
            println!("Sub theme not installed.");
            return Ok(());
        };

        fs::remove_file(&pkg.file)?;
        self.pkgs.retain(|pkg| pkg.id != id);

        if self.pkgs.is_empty() {
            self.clear_assets()?;
        }
        Ok(())
    }

    fn clear_assets(&mut self) -> io::Result<()> {
        debug_assert!(self.pkgs.is_empty());
        for path in self.assets.iter() {
            ShareDir::get(path)?.removed_by(&self.id)?;
        }
        self.assets.clear();
        Ok(())
    }
}
