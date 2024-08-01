use std::path::{self, PathBuf};
use std::{fs, io, path::Path};

use relative_path::RelativePathBuf;
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

pub(crate) mod defs;

/// Scan a directory recursively and return all files' paths relative to the directory.
pub(crate) fn scan_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<RelativePathBuf>> {
    let mut files = Vec::new();
    files.extend(
        WalkDir::new(&path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| RelativePathBuf::from_path(e.path().strip_prefix(&path).unwrap()).unwrap()),
    );
    Ok(files)
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
            ensure_dir(&dst_path)?;
            move_dir(path, &dst_path)?;
        } else if ty.is_file() {
            fs::rename(path, &dst_path)?;
        }
    }
    Ok(())
}

/// Ensure that a directory exists, failed if missing parent directories.
pub(crate) fn ensure_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    if !path.as_ref().exists() {
        fs::create_dir(path)?;
    }
    Ok(())
}

// An object that represents a file or a whole directory.
// note: can not be root directory
#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct Obj(PathBuf);

impl Obj {
    pub(crate) fn move_to<P: AsRef<Path>>(&mut self, dst: P) -> io::Result<()> {
        debug_assert!(dst.as_ref().is_dir());

        let dst_path = dst.as_ref().join(&self.0.file_name().unwrap());
        if self.0.is_dir() {
            if !dst_path.exists() {
                fs::create_dir(&dst_path)?;
            }
            for item in fs::read_dir(&self.0)? {
                Self(item?.path()).move_to(&dst_path)?;
            }
        } else {
            fs::rename(&self.0, &dst_path)?;
        }

        *self = Self(dst_path);
        Ok(())
    }

    pub(crate) fn remove(self) -> io::Result<()> {
        if self.0.is_dir() {
            fs::remove_dir_all(&self.0)
        } else {
            fs::remove_file(&self.0)
        }
    }

    pub(crate) fn name(&self) -> ObjName {
        ObjName(self.0.file_name().unwrap().to_str().unwrap().to_string())
    }
}

impl From<PathBuf> for Obj {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl AsRef<Path> for Obj {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct ObjName(String);

impl ObjName {
    pub(crate) fn base<P: AsRef<Path>>(&self, path: P) -> Obj {
        Obj(path.as_ref().join(&self.0))
    }
}