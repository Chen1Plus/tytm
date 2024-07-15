use std::{
    fs, io,
    path::{Path, PathBuf},
};

use lazy_static::lazy_static;

pub(crate) use tempfile::tempfile;
pub(crate) use tempfile::TempDir;

lazy_static! {
    /// The user's data directory.
    static ref DATA_DIR: PathBuf = dirs::data_dir().expect("Failed to find user's data directory");

    /// The directory where Typora themes are stored.
    pub static ref THEME_DIR: PathBuf = DATA_DIR.join("Typora").join("themes");

    /// The directory where TyTM stores its data.
    pub static ref TYTM_DIR: PathBuf = DATA_DIR.join("tytm");

    /// The directory where TyTM stores manifests.
    pub static ref MANIFEST_DIR: PathBuf = TYTM_DIR.join("manifest");
}

pub(crate) fn init_dirs() {
    assert!(
        THEME_DIR.exists() && THEME_DIR.is_dir(),
        "Typora themes directory not found"
    );
}

/// Recursively scan a directory and return all files.
/// You should ensure that `path` exists and is a directory.
pub(crate) fn scan_dir<P>(path: P) -> io::Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    debug_assert!(path.as_ref().exists() && path.as_ref().is_dir());

    let mut res = Vec::new();
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if path.is_dir() {
            res.extend(scan_dir(&path)?);
        } else if path.is_file() {
            res.push(path);
        }
    }
    Ok(res)
}

/// Move all files and directories from `src` to `dst`.  
/// You should ensure that both `src` and `dst` exist and are directories.  
/// If a file already exists, it will be overwritten.
pub(crate) fn move_dir<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    debug_assert!(src.as_ref().exists() && src.as_ref().is_dir());
    debug_assert!(dst.as_ref().exists() && dst.as_ref().is_dir());

    for item in fs::read_dir(src)? {
        let item = item?;

        let ty = item.file_type()?;
        let path = item.path();
        let dst_path = dst.as_ref().join(item.file_name());

        if ty.is_dir() {
            if !dst_path.exists() {
                fs::create_dir(&dst_path)?;
            }
            move_dir(path, &dst_path)?;
        } else if ty.is_file() {
            fs::rename(path, &dst_path)?;
        }
    }
    Ok(())
}
