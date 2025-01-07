use std::fs;
use std::path::{Path, PathBuf};

use reqwest::Url;
use tempfile::{tempdir, tempfile};
use zip::ZipArchive;

use crate::fsx;

pub enum UrlType {
    Git,
    Zip,
}

pub fn entry(url: Url, url_type: UrlType) -> anyhow::Result<()> {
    let mut tmp_file = tempfile()?;
    let tmp_dir = tempdir()?;

    println!("Downloading ...");
    reqwest::blocking::get(url)?.copy_to(&mut tmp_file)?;

    println!("Extracting...");
    ZipArchive::new(tmp_file)?.extract(&tmp_dir)?;

    let base = find_base_dir(tmp_dir.path())?;
    for entry in fs::read_dir(&base)? {
        let path = entry?.path();
        if path.is_dir() || path.extension() == Some("css".as_ref()) {
            fsx::Obj::from(path).move_to(fsx::defs::TYPORA_THEME.as_path())?;
        }
    }

    println!("Done");
    Ok(())
}

fn find_base_dir(from: &Path) -> anyhow::Result<PathBuf> {
    use std::cmp::Ordering;

    let mut files = fs::read_dir(&from)?.collect::<Result<Vec<_>, _>>()?;

    files.sort_by(|a, _| match a.path().is_file() {
        true => Ordering::Less,
        false => Ordering::Greater,
    });

    for f in files {
        match f.file_type()? {
            x if x.is_file() => {
                if let Some(ext) = f.path().extension() {
                    if ext.to_str() == Some("css") {
                        return Ok(f.path().parent().unwrap().to_owned());
                    }
                }
            }
            x if x.is_dir() => {
                if let Ok(res) = find_base_dir(&f.path()) {
                    return Ok(res);
                }
            }
            _ => (),
        }
    }

    Err(anyhow::anyhow!(
        "Unable to locate the base directory from {from:?}"
    ))
}
