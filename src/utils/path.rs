use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use crate::core::Error;

/// Convert any path-like type to a PathBuf
pub fn to_path_buf<P: AsRef<Path>>(path: P) -> PathBuf {
    path.as_ref().to_path_buf()
}

/// Convert any path-like type to a string, returning an error if the path is not valid UTF-8
pub fn to_string<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    path.as_ref().to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| Error::InvalidPath("Path contains invalid UTF-8".to_string()))
}

/// Convert any path-like type to a string, with a default value if the path is not valid UTF-8
pub fn to_string_lossy<P: AsRef<Path>>(path: P) -> String {
    path.as_ref().to_string_lossy().to_string()
}

/// Check if a path exists
pub fn exists<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Get the file name of a path as a String, returning None if there is no file name
/// or if the file name is not valid UTF-8
pub fn file_name<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref().file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
}

/// Get the extension of a path as a String, returning None if there is no extension
/// or if the extension is not valid UTF-8
pub fn extension<P: AsRef<Path>>(path: P) -> Option<String> {
    path.as_ref().extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_string())
}

/// Get the parent directory of a path, returning None if there is no parent
pub fn parent<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    path.as_ref().parent().map(|p| p.to_path_buf())
}

/// Append an extension to a path
pub fn with_extension<P: AsRef<Path>, S: AsRef<OsStr>>(path: P, extension: S) -> PathBuf {
    let mut path_buf = path.as_ref().to_path_buf();
    path_buf.set_extension(extension);
    path_buf
}

/// Join a path with one or more path components
pub fn join<P: AsRef<Path>, Q: AsRef<Path>>(path: P, other: Q) -> PathBuf {
    path.as_ref().join(other)
}

/// Convert a path to an absolute path
pub fn absolute<P: AsRef<Path>>(path: P) -> Result<PathBuf, Error> {
    std::fs::canonicalize(path)
        .map_err(|e| Error::InvalidPath(format!("Failed to get absolute path: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_path_functions() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        std::fs::write(&file_path, "test").unwrap();
        
        assert_eq!(to_path_buf(&file_path), file_path);
        assert_eq!(to_string(&file_path).unwrap(), file_path.to_str().unwrap());
        assert_eq!(to_string_lossy(&file_path), file_path.to_string_lossy());
        assert!(exists(&file_path));
        assert_eq!(file_name(&file_path).unwrap(), "test.txt");
        assert_eq!(extension(&file_path).unwrap(), "txt");
        assert_eq!(parent(&file_path).unwrap(), temp_dir.path());
        assert_eq!(with_extension(&file_path, "json"), temp_dir.path().join("test.json"));
        assert_eq!(join(&temp_dir.path(), "another.txt"), temp_dir.path().join("another.txt"));
    }
} 