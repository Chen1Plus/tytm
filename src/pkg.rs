use std::{
    fs::{self, File},
    io,
    path::{self, Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json as json;
use walkdir::WalkDir;

use crate::fsx::{self, dirs};

mod git;
mod zip;

pub(crate) fn update_manifest() -> Result<()> {
    let tmp_dir = fsx::TempDir::new()?;
    println!("Fetching manifest from GitHub...");
    git2::Repository::clone("https://github.com/Chen1Plus/tytm", &tmp_dir)?;
    fsx::move_dir(
        tmp_dir.path().join("manifest"),
        dirs::TYTM_MANIFEST.as_path(),
    )?;
    println!("Manifest updated.");
    Ok(())
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Package {
    id: String,
    name: String,
    version: String,
    source: Box<dyn Source>,
    assets: Vec<PathBuf>,
    pkgs: Vec<SubPackage>,
    default: Vec<String>,
}

impl Package {
    pub(crate) fn get(id: String) -> Result<Self> {
        json::from_reader(File::open(dirs::TYTM_MANIFEST.join(id + ".json"))?).map_err(Into::into)
    }

    pub(crate) fn install<S: AsRef<str>>(self, id: &[S]) -> Result<()> {
        let tmp_dir = fsx::TempDir::new()?;
        self.source.save_to(tmp_dir.path())?;

        println!("Installing...");
        let mut paths = Vec::new();
        for asset in self.assets.iter().map(|p| tmp_dir.path().join(p)) {
            let dst = dirs::TYPORA_THEME.join(asset.file_name().unwrap());
            if asset.is_dir() {
                paths.extend(
                    WalkDir::new(&asset)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .filter(|e| e.path().is_file())
                        .map(|e| {
                            dirs::TYPORA_THEME.join(
                                path::absolute(e.path())
                                    .unwrap()
                                    .strip_prefix(tmp_dir.path())
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

        let mut installed_subs = Vec::new();
        for pkg in self
            .pkgs
            .iter()
            .filter(|pkg| {
                id.iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<_>>()
                    .contains(&pkg.id.as_str())
            })
            .map(|pkg| pkg.install(&tmp_dir))
        {
            installed_subs.push(pkg?);
        }
        println!("Installed.");

        InstalledPackage {
            id: self.id.clone(),
            name: self.name.clone(),
            version: self.version.clone(),
            assets: paths,
            pkgs: installed_subs,
        }
        .save()
    }

    pub(crate) fn install_default(self) -> Result<()> {
        let id = self.default.clone();
        self.install(&id)
    }
}

#[typetag::serde(tag = "type", content = "value")]
trait Source {
    fn save_to(&self, path: &Path) -> Result<()>;
}

#[derive(Serialize, Deserialize)]
struct SubPackage {
    id: String,
    name: String,
    file: PathBuf,
}

impl SubPackage {
    // fn validate<P: AsRef<Path>>(&self, from: P) {
    //     let file = from.as_ref().join(&self.file);
    //     assert!(file.exists());
    //     assert!(file.is_file());
    //     assert!(file.ends_with(".css"));
    // }

    fn install<P: AsRef<Path>>(&self, from: P) -> Result<InstalledSubPackage> {
        let file = path::absolute(dirs::TYPORA_THEME.join(&self.file))?;
        fs::rename(from.as_ref().join(&self.file), &file)?;
        Ok(InstalledSubPackage {
            id: self.id.clone(),
            name: self.name.clone(),
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
    name: String,
    file: PathBuf,
}

impl InstalledPackage {
    pub(crate) fn get(id: String) -> Result<Self> {
        json::from_reader(File::open(dirs::TYPORA_MANIFEST.join(id + ".json"))?).map_err(Into::into)
    }

    pub(crate) fn save(&mut self) -> Result<()> {
        let path = dirs::TYPORA_MANIFEST.join(self.id.clone() + ".json");
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

    // panic if the sub theme not installed
    pub(crate) fn remove_sub(&mut self, id: &str) -> io::Result<()> {
        fs::remove_file(
            &self
                .pkgs
                .iter()
                .find(|pkg| pkg.id == id)
                .expect("Sub theme not installed")
                .file,
        )?;
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
