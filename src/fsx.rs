use std::{fs, io, path::Path};

pub(crate) use tempfile::tempfile;
pub(crate) use tempfile::TempDir;

#[cfg(debug_assertions)]
pub(crate) mod dirs {
    use std::path::PathBuf;

    lazy_static::lazy_static! {
        /// The user's data directory.
        static ref DATA: PathBuf = dirs::data_dir().expect("Failed to find user's data directory");

        static ref TYPORA: PathBuf = DATA.join("Typora");
        pub(crate) static ref TYPORA_THEME: PathBuf = PathBuf::from("debug-dirs").join("typora-themes");
        pub(crate) static ref TYPORA_MANIFEST: PathBuf = TYPORA_THEME.join("tytm-pkgs");

        static ref TYTM: PathBuf = DATA.join("tytm");
        pub(crate) static ref TYTM_MANIFEST: PathBuf = PathBuf::from("manifest");
    }

    pub(crate) fn init() {
        use super::ensure_dir;

        ensure_dir("debug-dirs").unwrap();
        ensure_dir(TYPORA_THEME.as_path()).unwrap();
        ensure_dir(TYPORA_MANIFEST.as_path()).unwrap();
    }
}

#[cfg(all(not(debug_assertions), target_os = "windows"))]
pub(crate) mod dirs {
    use std::path::PathBuf;

    use super::ensure_dir;

    lazy_static::lazy_static! {
        /// The user's data directory.
        static ref DATA: PathBuf = dirs::data_dir().expect("Failed to find user's data directory");

        static ref TYPORA: PathBuf = DATA.join("Typora");
        pub(crate) static ref TYPORA_THEME: PathBuf = TYPORA.join("themes");
        pub(crate) static ref TYPORA_MANIFEST: PathBuf = TYPORA_THEME.join("tytm-pkgs");

        static ref TYTM: PathBuf = DATA.join("tytm");
        pub(crate) static ref TYTM_MANIFEST: PathBuf = TYTM.join("manifest");
    }

    pub(crate) fn init() {
        assert!(
            TYPORA.exists() && TYPORA.is_dir(),
            "Typora directory not found"
        );
        assert!(
            TYPORA_THEME.exists() && TYPORA_THEME.is_dir(),
            "Typora themes directory not found"
        );

        ensure_dir(TYPORA_MANIFEST.as_path()).expect("Failed to create Typora manifest directory");
        assert!(TYPORA_MANIFEST.is_dir());

        ensure_dir(TYTM.as_path()).expect("Failed to create TyTM directory");
        assert!(TYTM.is_dir());

        ensure_dir(TYTM_MANIFEST.as_path()).expect("Failed to create TyTM manifest directory");
        assert!(TYTM_MANIFEST.is_dir());
    }
}

#[cfg(all(not(debug_assertions), target_os = "macos"))]
pub(crate) mod dirs {
    use std::path::PathBuf;

    use super::ensure_dir;

    lazy_static::lazy_static! {
        /// The user's data directory.
        static ref DATA: PathBuf = dirs::data_dir().expect("Failed to find user's data directory");

        static ref TYPORA: PathBuf = DATA.join("abnerworks.Typora");
        pub(crate) static ref TYPORA_THEME: PathBuf = TYPORA.join("themes");
        pub(crate) static ref TYPORA_MANIFEST: PathBuf = TYPORA_THEME.join("tytm-pkgs");

        static ref TYTM: PathBuf = DATA.join("tytm");
        pub(crate) static ref TYTM_MANIFEST: PathBuf = TYTM.join("manifest");
    }

    pub(crate) fn init() {
        assert!(
            TYPORA.exists() && TYPORA.is_dir(),
            "Typora directory not found"
        );
        assert!(
            TYPORA_THEME.exists() && TYPORA_THEME.is_dir(),
            "Typora themes directory not found"
        );

        ensure_dir(TYPORA_MANIFEST.as_path()).expect("Failed to create Typora manifest directory");
        assert!(TYPORA_MANIFEST.is_dir());

        ensure_dir(TYTM.as_path()).expect("Failed to create TyTM directory");
        assert!(TYTM.is_dir());

        ensure_dir(TYTM_MANIFEST.as_path()).expect("Failed to create TyTM manifest directory");
        assert!(TYTM_MANIFEST.is_dir());
    }
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

/// Ensure that a directory exists, create parent directories if missing.
fn ensure_dir_all<P: AsRef<Path>>(path: P) -> io::Result<()> {
    if !path.as_ref().exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}
