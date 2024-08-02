use std::collections::HashSet;
use std::fs::{self, File};
use std::io;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json as json;
use tempfile::{tempdir, TempDir};

use crate::fsx::{defs, Obj, ObjName, ShareDir};

mod source;

#[derive(Serialize, Deserialize)]
pub(crate) struct Manifest {
    id: String,
    name: String,
    version: String,
    source: Box<dyn source::Source>,
    assets: HashSet<ObjName>,
    pkgs: Vec<SubPackage>,
    default: HashSet<String>,
}

impl Manifest {
    pub(crate) fn update() -> Result<()> {
        let tmp_dir = tempdir()?;
        println!("Fetching manifests...");
        git2::Repository::clone("https://github.com/Chen1Plus/tytm", &tmp_dir)?;
        Obj::from(tmp_dir.path().join("manifest")).move_inside_to(defs::TYTM_MANIFEST.as_path())?;
        println!("Manifests updated.");
        Ok(())
    }

    pub(crate) fn get(id: &str) -> io::Result<Self> {
        Ok(json::from_reader(File::open(
            defs::TYTM_MANIFEST.join(id).with_extension("json"),
        )?)
        .expect("Invalid manifest."))
    }

    pub(crate) fn store_package(&self) -> Result<Package> {
        Ok(Package {
            id: self.id.clone(),
            name: self.name.clone(),
            version: self.version.clone(),
            base_path: self.source.download()?,
            assets: self.assets.clone(),
            pkgs: self.pkgs.clone(),
            default: self.default.clone(),
        })
    }
}

pub(crate) struct Package {
    id: String,
    name: String,
    version: String,
    base_path: TempDir,
    assets: HashSet<ObjName>,
    pkgs: Vec<SubPackage>,
    pub(crate) default: HashSet<String>,
}

impl Package {
    pub(crate) fn install(&self) -> Result<InstalledPackage> {
        for asset in &self.assets {
            let dst = asset.base(defs::TYPORA_THEME.as_path());
            let mut real_asset = asset.base(&self.base_path);

            // debug_assert!(real_asset.is_dir());

            ShareDir::get(&dst)?.used_by(&self.id)?;
            real_asset.move_to(defs::TYPORA_THEME.as_path())?;
        }

        Ok(InstalledPackage {
            id: self.id.clone(),
            name: self.name.clone(),
            version: self.version.clone(),
            assets: self
                .assets
                .iter()
                .map(|x| x.base(defs::TYPORA_THEME.as_path()))
                .collect(),
            pkgs: Vec::new(),
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct SubPackage {
    id: String,
    file: ObjName,
}

impl SubPackage {
    fn install(&self, from: &Path) -> Result<InstalledSubPackage> {
        let file = self.file.base(defs::TYPORA_THEME.as_path());
        fs::rename(self.file.base(from), &file)?;
        Ok(InstalledSubPackage {
            id: self.id.clone(),
            file,
        })
    }
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct InstalledPackage {
    id: String,
    name: String,
    version: String,
    assets: HashSet<Obj>,
    pkgs: Vec<InstalledSubPackage>,
}

#[derive(Serialize, Deserialize)]
struct InstalledSubPackage {
    id: String,
    file: Obj,
}

impl InstalledPackage {
    pub(crate) fn get(id: &str) -> io::Result<Self> {
        Ok(json::from_reader(File::open(
            defs::TYPORA_MANIFEST.join(id).with_extension("json"),
        )?)
        .expect("Invalid manifest."))
    }

    pub(crate) fn save(&mut self) -> Result<()> {
        debug_assert!(!self.pkgs.is_empty(), "Try to save an empty package.");

        let path = defs::TYPORA_MANIFEST.join(&self.id).with_extension("json");
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

    pub(crate) fn add_sub(&mut self, id: &str, from: &Package) -> Result<()> {
        self.pkgs.push(
            from.pkgs
                .iter()
                .find(|pkg| pkg.id == id)
                .expect("Sub theme not found")
                .install(&from.base_path.path())?,
        );
        Ok(())
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
