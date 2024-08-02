use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;
use std::{fs, io, path::Path};

use serde::{Deserialize, Serialize};
use serde_json as json;

pub(crate) mod defs;

// An object that represents a file or a whole directory.
// note: can not be root directory
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

// An object that represents a file or a whole directory, which has a UTF-8 name.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    pub(crate) fn get<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = path.as_ref().join(".tytm.fsx.lock");

        Ok(if file.is_file() {
            let dir: ShareDir = json::from_reader(File::open(&file)?)?;
            if dir.path != path.as_ref() {
                panic!("Got a broken ShareDir at {:?}", path.as_ref())
            }
            dir
        } else {
            if !path.as_ref().is_dir() {
                fs::create_dir_all(&path)?;
            }

            Self {
                path: path.as_ref().to_path_buf(),
                used_by: HashSet::new(),
            }
        })
    }

    pub(crate) fn used_by<S: ToString>(&mut self, by: S) -> io::Result<()> {
        self.used_by.insert(by.to_string());
        self.save()
    }

    pub(crate) fn removed_by<S: AsRef<str>>(&mut self, by: S) -> io::Result<()> {
        self.used_by.retain(|x| x != by.as_ref());
        if self.used_by.is_empty() {
            Obj::from(self.path.clone()).remove()
        } else {
            self.save()
        }
    }

    // this will return error if you tend to remove a deleted directory
    fn save(&self) -> io::Result<()> {
        let file = self.path.join(".tytm.fsx.lock");
        if file.is_file() {
            fs::remove_file(&file)?;
        }
        json::to_writer(File::create(&file)?, self).map_err(Into::into)
    }
}
