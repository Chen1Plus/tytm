use std::path::PathBuf;

use super::ensure_dir;

#[cfg(debug_assertions)]
lazy_static::lazy_static! {
    /// The user's data directory.
    static ref DATA: PathBuf = dirs::data_dir().expect("Failed to find user's data directory");

    static ref TYPORA: PathBuf = DATA.join("Typora");
    pub(crate) static ref TYPORA_THEME: PathBuf = PathBuf::from("debug-dirs").join("typora-themes");
    pub(crate) static ref TYPORA_MANIFEST: PathBuf = TYPORA_THEME.join("tytm-pkgs");

    static ref TYTM: PathBuf = DATA.join("tytm");
    pub(crate) static ref TYTM_MANIFEST: PathBuf = PathBuf::from("manifest");
}

#[cfg(debug_assertions)]
pub(crate) fn init() {
    ensure_dir("debug-dirs").unwrap();
    ensure_dir(TYPORA_THEME.as_path()).unwrap();
    ensure_dir(TYPORA_MANIFEST.as_path()).unwrap();
}

#[cfg(all(not(debug_assertions), target_os = "windows"))]
lazy_static::lazy_static! {
    /// The user's data directory.
    static ref DATA: PathBuf = dirs::data_dir().expect("Failed to find user's data directory");

    static ref TYPORA: PathBuf = DATA.join("Typora");
    pub(crate) static ref TYPORA_THEME: PathBuf = TYPORA.join("themes");
    pub(crate) static ref TYPORA_MANIFEST: PathBuf = TYPORA_THEME.join("tytm-pkgs");

    static ref TYTM: PathBuf = DATA.join("tytm");
    pub(crate) static ref TYTM_MANIFEST: PathBuf = TYTM.join("manifest");
}

#[cfg(not(debug_assertions))]
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

#[cfg(all(not(debug_assertions), target_os = "macos"))]
lazy_static::lazy_static! {
    /// The user's data directory.
    static ref DATA: PathBuf = dirs::data_dir().expect("Failed to find user's data directory");

    static ref TYPORA: PathBuf = DATA.join("abnerworks.Typora");
    pub(crate) static ref TYPORA_THEME: PathBuf = TYPORA.join("themes");
    pub(crate) static ref TYPORA_MANIFEST: PathBuf = TYPORA_THEME.join("tytm-pkgs");

    static ref TYTM: PathBuf = DATA.join("tytm");
    pub(crate) static ref TYTM_MANIFEST: PathBuf = TYTM.join("manifest");
}
