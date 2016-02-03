use std::{path, fs};
use path::{Path};
use fs::{FSRead, Result};

/// A native, local filesystem.
///
/// This object implements all the FS traits, and simply passes
/// through operations to the local file system under a prefix.
pub struct Native {
    inner: path::PathBuf
}

impl Native {
    pub fn new<P: AsRef<path::Path>>(path: P) -> Native {
        Native { inner: path.as_ref().to_path_buf() }
    }

    fn path<P: AsRef<Path>>(&self, path: P) -> path::PathBuf {
        let mut p = self.inner.clone();
        for part in path.as_ref() {
            p.push(part.as_str());
        }
        return p;
    }
}

impl FSRead for Native {
    type ReadFile = fs::File;

    fn open<P: AsRef<Path>>(&self, path: P) -> Result<fs::File> {
        fs::File::open(self.path(path))
    }

    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        self.path(path).exists()
    }

    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.path(path).is_file()
    }

    fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        self.path(path).is_dir()
    }
}
