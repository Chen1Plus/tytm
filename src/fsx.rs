use std::{
    fs, io,
    path::{Path, PathBuf},
};

use lazy_static::lazy_static;

pub(crate) use dirs::data_dir;
pub(crate) use tempfile::tempfile;
pub(crate) use tempfile::TempDir;

lazy_static! {
    pub static ref THEME_DIR: PathBuf = data_dir().unwrap().join("Typora/themes");
}

/// Move all files and directories from `src` to `dst`.  
/// You should ensure that both `src` and `dst` exist and are directories.  
/// If a file already exists, it will be overwritten.
pub(crate) fn move_dir<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let src = src.as_ref();
    let dst = dst.as_ref();

    debug_assert!(src.exists() && src.is_dir());
    debug_assert!(dst.exists() && dst.is_dir());

    for item in fs::read_dir(src)? {
        let item = item?;

        let ty = item.file_type()?;
        let path = item.path();
        let dst_path = dst.join(item.file_name());

        if ty.is_dir() {
            fs::create_dir(&dst_path)?;
            move_dir(path, &dst_path)?;
        } else if ty.is_file() {
            fs::rename(path, &dst_path)?;
        }
    }
    Ok(())
}
