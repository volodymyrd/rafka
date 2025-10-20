use once_cell::sync::Lazy;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tempfile::{Builder, TempDir};

/// Global, lazy-initialized vector to hold the TempDir guards.
/// Mutex is used to ensure thread-safe access for adding new guards.
static TEMP_DIR_GUARDS: Lazy<Mutex<Vec<TempDir>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// Creates a temporary directory in the specified parent directory with the given prefix.
///
/// This function creates a [TempDir] RAII guard and registers it in a **static, process-global**
/// collection to ensure the directory is automatically deleted when the process terminates.
///
/// # Arguments
///
/// * `parent` - The optional parent directory path. If `None`, the system's default
///              temporary-file directory is used.
/// * `prefix` - The optional prefix for the temporary directory's name. If `None`,
///              the default prefix `"rafka-"` is used.
///
/// # Returns
///
/// Returns an `io::Result<PathBuf>` containing the path to the newly created temporary directory.
/// The cleanup is handled by the internal static guard.
pub fn temp_directory(parent: Option<&Path>, prefix: Option<&str>) -> io::Result<PathBuf> {
    let final_prefix = prefix.unwrap_or("rafka-");

    // Create the TempDir guard using the Builder pattern
    let temp_dir_guard = match parent {
        Some(p) => Builder::new().prefix(final_prefix).tempdir_in(p)?,
        None => Builder::new().prefix(final_prefix).tempdir()?,
    };

    let path = temp_dir_guard.path().to_owned();

    TEMP_DIR_GUARDS.lock().unwrap().push(temp_dir_guard);

    Ok(path)
}

/// Creates a temporary directory in the default system temporary-file directory
/// with the default prefix `"rafka-"`.
///
/// # Returns
///
/// Returns an `io::Result<PathBuf>` containing the path to the newly created temporary directory.
pub fn temp_directory_default() -> io::Result<PathBuf> {
    temp_directory(None, None)
}

/// Creates a temporary directory in the default system temporary-file directory,
/// allowing a custom prefix.
///
/// # Arguments
///
/// * `prefix` - The optional prefix for the temporary directory's name. If `None`,
///              the default prefix `"rafka-"` is used.
///
/// # Returns
///
/// Returns an `io::Result<PathBuf>` containing the path to the newly created temporary directory.
pub fn temp_directory_with_prefix(prefix: Option<&str>) -> io::Result<PathBuf> {
    temp_directory(None, prefix)
}

/// Creates a temporary directory under the specified root directory.
///
/// If the root directory does not exist, it will be created. The temporary directory is
/// guaranteed to be cleaned up when the process exits.
///
/// # Arguments
///
/// * `root` - The path to the directory under which the temporary directory should be created.
///
/// # Returns
///
/// Returns an `io::Result<PathBuf>` containing the path to the temporary directory created within `root`.
pub fn temp_relative_dir(root: &str) -> io::Result<PathBuf> {
    let root = Path::new(root);
    // Ensure the root directory exists.
    std::fs::create_dir_all(root)?;

    temp_directory(Some(root), None)
}
