use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};

pub(crate) mod defs;

/// Move all files and directories from `src` to `dst`.
/// You should ensure that both `src` and `dst` exist and are directories.
/// If a file already exists, it will be overwritten.
#[deprecated]
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
#[deprecated]
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
    pub(crate) fn move_to<P: AsRef<Path>>(&mut self, to: P) -> io::Result<()> {
        debug_assert!(to.as_ref().is_dir());

        let dst_path = to.as_ref().join(&self.0.file_name().unwrap());
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

    // move all files and directories inside the object to `to`
    // if object is a file, it will be moved to `to` as well
    pub(crate) fn move_inside_to<P: AsRef<Path>>(&mut self, to: P) -> io::Result<()> {
        debug_assert!(to.as_ref().is_dir());

        if self.0.is_dir() {
            for item in fs::read_dir(&self.0)? {
                Self(item?.path()).move_to(&to)?;
            }
            *self = Self(to.as_ref().to_owned());
        } else {
            let dst_path = to.as_ref().join(&self.0.file_name().unwrap());
            fs::rename(&self.0, &dst_path)?;
            *self = Self(dst_path);
        }
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

#[derive(Serialize, Deserialize)]
pub(crate) struct ShareDir {
    path: PathBuf,
    used_by: HashSet<String>,
}

impl ShareDir {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            used_by: HashSet::new(),
        }
    }

    pub(crate) fn get<P: AsRef<Path>>(path: P, id: String) -> io::Result<Self> {
        let mut ret;
        let file = path.as_ref().join(".tytm.fsx.lock");
        if path.as_ref().is_dir() && file.is_file() {
            ret = serde_json::from_reader(File::open(file)?).unwrap();
        } else {
            ret = Self::new(path.as_ref().to_path_buf());
        }
        ret.used_by.insert(id);
        Ok(ret)
    }

    pub(crate) fn remove(&mut self, by: &str) -> io::Result<()> {
        self.used_by.retain(|x| x != by);
        if self.used_by.is_empty() {
            Obj::from(self.path.clone()).remove()?;
        }
        Ok(())
    }

    pub(crate) fn save(&self) -> io::Result<()> {
        if !self.used_by.is_empty() {
            let file = self.path.join(".tytm.fsx.lock");
            serde_json::to_writer(File::create(file)?, self)?;
        }
        Ok(())
    }

    pub(crate) fn path(&self) -> &Path {
        &self.path
    }
}
