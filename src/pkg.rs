use std::fs::{self, File};
use std::io;
use std::path::{self, Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json as json;
use walkdir::WalkDir;

use crate::fsx::{self, dirs};

mod git;
mod zip;

#[derive(Serialize, Deserialize)]
pub(crate) struct Manifest {
    id: String,
    name: String,
    version: String,
    source: Box<dyn Source>,
    assets: Vec<PathBuf>,
    pkgs: Vec<SubPackage>,
    default: Vec<String>,
}

impl Manifest {
    pub(crate) fn update() -> Result<()> {
        let tmp_dir = fsx::TempDir::new()?;
        println!("Fetching manifests...");
        git2::Repository::clone("https://github.com/Chen1Plus/tytm", &tmp_dir)?;
        fsx::move_dir(
            tmp_dir.path().join("manifest"),
            dirs::TYTM_MANIFEST.as_path(),
        )?;
        println!("Manifests updated.");
        Ok(())
    }

    pub(crate) fn get(id: &str) -> io::Result<Self> {
        Ok(json::from_reader(File::open(
            dirs::TYTM_MANIFEST.join(id).with_extension("json"),
        )?)
        .expect("Invalid manifest."))
    }

    pub(crate) fn store_package<P: AsRef<Path>>(&self, path: P) -> Result<Package> {
        let path = path.as_ref();
        self.source.save_to(path)?;

        Ok(Package {
            id: self.id.clone(),
            name: self.name.clone(),
            version: self.version.clone(),
            assets: self.assets.iter().map(|p| path.join(p)).collect(),
            pkgs: self
                .pkgs
                .iter()
                .map(|p| SubPackage {
                    id: p.id.clone(),
                    file: path.join(&p.file),
                })
                .collect(),
            default: self.default.clone(),
        })
    }
}

#[typetag::serde(tag = "type", content = "value")]
trait Source {
    fn save_to(&self, path: &Path) -> Result<()>;
}

pub(crate) struct Package {
    id: String,
    name: String,
    version: String,
    assets: Vec<PathBuf>,
    pkgs: Vec<SubPackage>,
    pub(crate) default: Vec<String>,
}

impl Package {
    pub(crate) fn install(&self) -> Result<InstalledPackage> {
        let mut paths = Vec::new();
        for asset in &self.assets {
            let dst = dirs::TYPORA_THEME.join(asset.file_name().unwrap());
            if asset.is_dir() {
                let parent = asset.parent().unwrap();
                paths.extend(
                    WalkDir::new(&asset)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_file())
                        .map(|e| {
                            dirs::TYPORA_THEME.join(
                                path::absolute(e.path())
                                    .unwrap()
                                    .strip_prefix(parent)
                                    .unwrap(),
                            )
                        }),
                );
                fsx::ensure_dir(&dst)?;
                fsx::move_dir(asset, &dst)?;
            } else if asset.is_file() {
                fs::rename(asset, &dst)?;
                paths.push(dst);
            }
        }

        Ok(InstalledPackage {
            id: self.id.clone(),
            name: self.name.clone(),
            version: self.version.clone(),
            assets: paths,
            pkgs: Vec::new(),
        })
    }
}

#[derive(Serialize, Deserialize)]
struct SubPackage {
    id: String,
    file: PathBuf,
}

impl SubPackage {
    // fn validate<P: AsRef<Path>>(&self, from: P) {
    //     let file = from.as_ref().join(&self.file);
    //     assert!(file.exists());
    //     assert!(file.is_file());
    //     assert!(file.ends_with(".css"));
    // }

    fn install(&self) -> Result<InstalledSubPackage> {
        let file = path::absolute(dirs::TYPORA_THEME.join(&self.file.file_name().unwrap()))?;
        fs::rename(&self.file, &file)?;
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
    assets: Vec<PathBuf>,
    pkgs: Vec<InstalledSubPackage>,
}

#[derive(Serialize, Deserialize)]
struct InstalledSubPackage {
    id: String,
    file: PathBuf,
}

impl InstalledPackage {
    pub(crate) fn get(id: &str) -> io::Result<Self> {
        Ok(json::from_reader(File::open(
            dirs::TYPORA_MANIFEST.join(id).with_extension("json"),
        )?)
        .expect("Invalid manifest."))
    }

    pub(crate) fn save(&mut self) -> Result<()> {
        let path = dirs::TYPORA_MANIFEST.join(&self.id).with_extension("json");
        if path.exists() {
            fs::remove_file(&path)?;
        }

        if self.pkgs.is_empty() {
            self.clear_assets()?;
        } else {
            json::to_writer(File::create(&path)?, self)?;
        }
        Ok(())
    }

    pub(crate) fn remove(&mut self) -> io::Result<()> {
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
                .install()?,
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
            fs::remove_file(path)?;
        }
        self.assets.clear();
        Ok(())
    }
}
