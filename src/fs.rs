use std::{io, result};
use path::{Path, PathBuf};

pub use std::io::{Error};

/// The result type for all filesystem IO.
pub type Result<T> = result::Result<T, Error>;

/// Possible file types.
pub enum FileType {
    Dir,
    File,
}

impl FileType {
    pub fn dir() -> FileType {
        FileType::Dir
    }

    pub fn file() -> FileType {
        FileType::File
    }
    
    pub fn is_dir(&self) -> bool {
        match self {
            &FileType::Dir => true,
            _ => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            &FileType::File => true,
            _ => false,
        }
    }
}

/// An iterator over directory entries.
pub struct DirEntries<'a, T: 'a + ?Sized + FSRead<'a>> {
    inner: T::ReadDir,
    parent: &'a T,
}

impl<'a, T: ?Sized + FSRead<'a>> Iterator for DirEntries<'a, T> {
    type Item = QPath<'a, T>;

    fn next(&mut self) -> Option<QPath<'a, T>> {
        if let Some(p) = self.inner.next() {
            Some(self.parent.qualified(p))
        } else {
            None
        }
    }
}

/// A Qualified path, a path tied to a particular filesystem.
pub struct QPath<'a, T: 'a + ?Sized> {
    path: PathBuf,
    parent: &'a T,
}

impl<'a, T: ?Sized + FSRead<'a>> AsRef<Path> for QPath<'a, T> {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl<'a, T: ?Sized + FSRead<'a>> QPath<'a, T> {
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    pub fn open(&self) -> Result<T::ReadFile> {
        self.parent.open(&self.path)
    }

    pub fn file_type(&self) -> Result<FileType> {
        self.parent.file_type(&self.path)
    }

    pub fn exists(&self) -> bool {
        self.parent.exists(&self.path)
    }

    pub fn is_file(&self) -> bool {
        self.parent.is_file(&self.path)
    }

    pub fn is_dir(&self) -> bool {
        self.parent.is_dir(&self.path)
    }

    pub fn read_dir(&self) -> Result<T::ReadDir> {
        self.parent.read_dir(&self.path)
    }
}

/// Operations for readable file systems.
pub trait FSRead<'a> : 'a{
    fn qualified<P: AsRef<Path>>(&'a self, path: P) -> QPath<'a, Self> {
        QPath { path: path.as_ref().to_owned(), parent: self }
    }

    type ReadFile: io::Read;
    fn open<P: AsRef<Path>>(&self, path: P) -> Result<Self::ReadFile>;

    // fn metadata<P: AsRef<Path>>(&self, path: P) -> Result<Metadata>;
    fn file_type<P: AsRef<Path>>(&self, path: P) -> Result<FileType>;
    
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool {
        self.file_type(path).is_ok()
    }
    
    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.file_type(path).map(|t| t.is_file()).unwrap_or(false)
    }
    
    fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool {
        self.file_type(path).map(|t| t.is_dir()).unwrap_or(false)
    }

    type ReadDir: Iterator<Item=QPath<'a, Self>>;
    fn read_dir<P: AsRef<Path>>(&'a self, path: P) -> Result<Self::ReadDir>;
}
