use std::{fs::File, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json as json;

use crate::pkg::{InstalledPackage, Package};

#[derive(Serialize, Deserialize)]
pub(crate) struct PkgList {
    version: u8,
    pkgs: Vec<Package>,
}

impl PkgList {
    pub(crate) fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        json::from_reader(File::open(path)?).map_err(Into::into)
    }

    pub(crate) fn get_pkg(&self, id: &str) -> Option<&Package> {
        self.pkgs.iter().find(|&pkg| pkg.id == id)
    }
}

#[derive(Default, Serialize, Deserialize)]
pub(crate) struct InstalledPkgList {
    version: u8,
    pkgs: Vec<InstalledPackage>,
}

impl InstalledPkgList {
    pub(crate) fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        json::from_reader(File::open(path)?).map_err(Into::into)
    }

    pub(crate) fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        json::to_writer(File::create(path)?, self).map_err(Into::into)
    }

    pub(crate) fn get_pkg(&self, id: &str) -> Option<&InstalledPackage> {
        self.pkgs.iter().find(|&pkg| pkg.id == id)
    }

    pub(crate) fn add_pkg(&mut self, pkg: InstalledPackage) {
        self.pkgs.push(pkg);
    }

    pub(crate) fn rm_pkg(&mut self, id: &str) {
        self.pkgs.retain(|pkg| pkg.id != id);
    }
}
