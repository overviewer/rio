use std::{io, result};
use path::{Path};

pub use std::io::{Error};

/// The result type for all filesystem IO.
pub type Result<T> = result::Result<T, Error>;

/// Operations for readable file systems.
pub trait FSRead {
    type ReadFile: io::Read;

    fn open<P: AsRef<Path>>(&self, path: P) -> Result<Self::ReadFile>;
    fn exists<P: AsRef<Path>>(&self, path: P) -> bool;
    fn is_file<P: AsRef<Path>>(&self, path: P) -> bool;
    fn is_dir<P: AsRef<Path>>(&self, path: P) -> bool;
}
