use std::{path, fs, io};
use std::convert::From;
use path::{Path, PathBuf};
use fs::{FSRead, Result, FileType, QPath};

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

    fn unpath<P: AsRef<path::Path>>(&self, path: P) -> Option<PathBuf> {
        path.as_ref().relative_from(&self.inner).and_then(|p| p.to_str()).map(From::from)
    }
}

pub struct ReadDir<'a> {
    iter: fs::ReadDir,
    parent: &'a Native,
}

impl<'a> Iterator for ReadDir<'a> {
    type Item = QPath<'a, Native>;

    fn next(&mut self) -> Option<QPath<'a, Native>> {
        loop {
            if let Some(res) = self.iter.next() {
                let conv = res.ok().and_then(|r| self.parent.unpath(r.path()));
                if let Some(p) = conv {
                    return Some(self.parent.qualified(p));
                }
            } else {
                return None;
            }
        }
    }
}

impl<'a> FSRead<'a> for Native {
    type ReadFile = fs::File;

    fn open<P: AsRef<Path>>(&self, path: P) -> Result<fs::File> {
        fs::File::open(self.path(path))
    }

    fn file_type<P: AsRef<Path>>(&self, path: P) -> Result<FileType> {
        let p = self.path(path);
        if p.exists() {
            if p.is_file() {
                return Ok(FileType::File);
            } else if p.is_dir() {
                return Ok(FileType::Dir);
            }
        }
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found."));
    }

    type ReadDir = ReadDir<'a>;

    fn read_dir<P: AsRef<Path>>(&self, path: P) -> Result<ReadDir> {
        self.path(path).read_dir().map(|dirs| ReadDir { iter: dirs, parent: self })
    }
}
