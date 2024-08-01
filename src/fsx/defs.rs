use std::{path::PathBuf, sync::LazyLock};

use super::ensure_dir;

/// The user's data directory.
static DATA: LazyLock<PathBuf> =
    LazyLock::new(|| dirs::data_dir().expect("Failed to find user's data directory"));
/// The user's cache directory.
static CACHE: LazyLock<PathBuf> =
    LazyLock::new(|| dirs::cache_dir().expect("Failed to find user's cache directory"));

#[cfg(debug_assertions)]
static TYPORA: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("debug-dirs").join("Typora"));
#[cfg(all(not(debug_assertions), target_os = "windows"))]
static TYPORA: LazyLock<PathBuf> = LazyLock::new(|| DATA.join("Typora"));
#[cfg(all(not(debug_assertions), target_os = "macos"))]
static TYPORA: LazyLock<PathBuf> = LazyLock::new(|| DATA.join("abnerworks.Typora"));

pub(crate) static TYPORA_THEME: LazyLock<PathBuf> = LazyLock::new(|| TYPORA.join("themes"));
pub(crate) static TYPORA_MANIFEST: LazyLock<PathBuf> =
    LazyLock::new(|| TYPORA_THEME.join("tytm-pkgs"));

static TYTM: LazyLock<PathBuf> = LazyLock::new(|| DATA.join("tytm"));

#[cfg(debug_assertions)]
pub(crate) static TYTM_MANIFEST: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("manifest"));
#[cfg(not(debug_assertions))]
pub(crate) static TYTM_MANIFEST: LazyLock<PathBuf> = LazyLock::new(|| TYTM.join("manifest"));

#[cfg(debug_assertions)]
pub(crate) fn init() {
    ensure_dir("debug-dirs").unwrap();
    ensure_dir(TYPORA.as_path()).unwrap();
    ensure_dir(TYPORA_THEME.as_path()).unwrap();
    ensure_dir(TYPORA_MANIFEST.as_path()).unwrap();
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
