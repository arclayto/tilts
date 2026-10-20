//! On-disk cache of downloaded data, as Parquet, under the platform data directory
//! (`~/.local/share/tilts` on Linux, `~/Library/Application Support/tilts` on macOS,
//! `%APPDATA%\tilts` on Windows).
//!
//! Keys are plain strings like `french/us/ff3/monthly` or `prices/tiingo/VTI`;
//! each maps to one Parquet file. There is no TTL yet: `tilts cache clear` is the
//! refresh mechanism until M2 adds staleness checks.

use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use polars::prelude::DataFrame;

use crate::error::{Error, Result};

/// A directory of Parquet files addressed by string keys.
#[derive(Debug, Clone)]
pub struct Cache {
    root: PathBuf,
}

impl Cache {
    /// A cache rooted at the platform's data directory for `tilts`.
    pub fn default_location() -> Result<Self> {
        let dirs = ProjectDirs::from("", "", "tilts").ok_or_else(|| {
            Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "no home directory to put a cache in",
            ))
        })?;
        Ok(Self::at(dirs.data_dir()))
    }

    /// A cache rooted at an explicit directory (tests, `--cache-dir`).
    pub fn at(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    /// Where this cache lives.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Path of the Parquet file for `key`.
    pub fn path_for(&self, key: &str) -> PathBuf {
        self.root.join(format!("{key}.parquet"))
    }

    /// Load a frame if `key` is cached.
    pub fn get(&self, key: &str) -> Result<Option<DataFrame>> {
        let _ = key;
        todo!("M0: read Parquet if present")
    }

    /// Store a frame under `key`, creating directories as needed.
    pub fn put(&self, key: &str, df: &DataFrame) -> Result<()> {
        let _ = (key, df);
        todo!("M0: write Parquet atomically (temp file + rename)")
    }

    /// Delete everything under the cache root.
    pub fn clear(&self) -> Result<()> {
        if self.root.exists() {
            std::fs::remove_dir_all(&self.root)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_map_to_nested_parquet_paths() {
        let c = Cache::at("/tmp/tilts-test");
        assert_eq!(
            c.path_for("french/us/ff3/monthly"),
            PathBuf::from("/tmp/tilts-test/french/us/ff3/monthly.parquet")
        );
    }
}
