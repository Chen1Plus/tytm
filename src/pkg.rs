use std::{
    fs::{self, File},
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

    pub(crate) fn install(self, id: &[&str]) -> Result<()> {
        let tmp_dir = fsx::TempDir::new()?;
        self.source.save_to(tmp_dir.path())?;

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
                if !dst.exists() {
                    fs::create_dir(&dst)?;
                }
                fsx::move_dir(asset, dst)?;
            } else if asset.is_file() {
                fs::rename(asset, &dst)?;
                paths.push(dst);
            }
        }

        let mut installed_subs = Vec::new();
        for pkg in self
            .pkgs
            .iter()
            .filter(|pkg| id.contains(&pkg.id.as_str()))
            .map(|pkg| pkg.install(&tmp_dir))
        {
            installed_subs.push(pkg?);
        }

        json::to_writer(
            File::create(dirs::TYPORA_MANIFEST.join(self.id.clone() + ".json"))?,
            &(InstalledPackage {
                id: self.id.clone(),
                name: self.name.clone(),
                version: self.version.clone(),
                assets: paths,
                pkgs: installed_subs,
            }),
        )
        .map_err(Into::into)
    }

    pub(crate) fn install_default(self) -> Result<()> {
        let id = self.default.clone();
        self.install(&id.iter().map(|s| s.as_str()).collect::<Vec<_>>())
    }
}

#[typetag::serde(tag = "type", content = "value")]
trait Source {
    fn save_to(&self, path: &Path) -> Result<()>;

    fn install(&self) -> Result<Vec<PathBuf>>;
}

#[derive(Serialize, Deserialize)]
struct SubPackage {
    id: String,
    name: String,
    file: PathBuf,
}

impl SubPackage {
    fn validate<P: AsRef<Path>>(&self, from: P) {
        let file = from.as_ref().join(&self.file);
        assert!(file.exists());
        assert!(file.is_file());
        assert!(file.ends_with(".css"));
    }

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
pub(crate) struct InstalledSubPackage {
    id: String,
    name: String,
    file: PathBuf,
}

impl InstalledPackage {
    pub(crate) fn get(id: String) -> Result<Self> {
        json::from_reader(File::open(dirs::TYPORA_MANIFEST.join(id + ".json"))?).map_err(Into::into)
    }

    // pub(crate) fn uninstall(&self) -> Result<()> {
    //     for file in &self.added_files {
    //         debug_assert!(file.exists() && file.is_file());
    //         fs::remove_file(file)?;
    //     }
    //     fs::remove_file(dirs::TYPORA_MANIFEST.join(self.id.clone() + ".json"))?;
    //     Ok(())
    // }
}
