use std::rc::Rc;
use std::{fs::File, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json as json;

use crate::fsx;
use crate::pkg::{InstalledPackage, Package};
// use crate::source::Source;

// #[derive(Serialize, Deserialize)]
// pub(crate) struct Registry {
//     version: u8,
//     themes: Vec<Theme>,
// }

// impl Registry {
//     pub(crate) fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
//         json::from_reader(File::open(path)?).map_err(Into::into)
//     }

//     pub(crate) fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<()> {
//         json::to_writer(File::create(path)?, self).map_err(Into::into)
//     }

//     pub(crate) fn get_theme(&self, id: &str) -> Option<&Theme> {
//         self.themes.iter().find(|&theme| theme.id == id)
//     }

//     pub(crate) fn add_theme(&mut self, theme: Theme) {
//         self.themes.push(theme);
//     }
// }

// impl Default for Registry {
//     fn default() -> Self {
//         Self {
//             version: 1,
//             themes: Vec::new(),
//         }
//     }
// }

// #[derive(Clone, Serialize, Deserialize)]
// pub(crate) struct Theme {
//     id: String,
//     name: String,
//     version: String,
//     source: Rc<dyn Source>,
// }

// impl Theme {
//     pub(crate) fn install(&self) -> Result<()> {
//         let theme_dir = fsx::data_dir().unwrap().join("Typora").join("themes");
//         self.source.save_to(&theme_dir)?;
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::Registry;

//     #[test]
//     fn read_registry() {
//         let registry = Registry::from_file("try.json").unwrap();
//         registry.themes.iter().nth(0).unwrap().install().unwrap();
//     }
// }

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
