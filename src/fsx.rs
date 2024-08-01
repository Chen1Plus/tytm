use std::path;
use std::{fs, io, path::Path};

use relative_path::RelativePathBuf;
use walkdir::WalkDir;

pub(crate) use tempfile::tempfile;
pub(crate) use tempfile::TempDir;

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
