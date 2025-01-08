use std::path::PathBuf;
use std::{fs, io, path::Path};

// An object that represents a file or a whole directory.
// note: can not be root directory
#[derive(Clone, PartialEq, Eq, Hash)]
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
