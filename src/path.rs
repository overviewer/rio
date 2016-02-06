// mostly follows the std::path type
// https://doc.rust-lang.org/src/std/path.rs.html
// except all paths use / as seperator, and no paths are relative. Yes, really.
// ("/a/b" is the same as "a/b", Path is essentially isomorphic to &[&str])

use std::{mem, fmt};
use std::ops::{Deref};
use std::borrow::{Borrow, ToOwned, Cow};

/// An owned path string.
///
/// See [Path](struct.Path.html) for details.
#[derive(Clone)]
pub struct PathBuf {
    inner: String
}

/// A path reference.
///
/// This path type is modeled on `std::path::Path`, with two crucial
/// differences: it always uses `/` as a seperator, and all paths are
/// treated as relative. So, "/a/b" and "a/b" represent the same
/// path. If you'd like, you can think of each path as a sequence of
/// strings, with no other structure.
pub struct Path {
    inner: str
}

/// An iterator over components of a path.
///
/// This is produced by [Path::components()](struct.Path.html#method.components).
#[derive(Clone)]
pub struct Components<'a> {
    path: &'a [u8],
    i: usize,
    j: usize,
}

impl<'a> Components<'a> {
    fn trim_left(&mut self) -> usize {
        while self.i < self.j && self.path[self.i] == b'/' {
            self.i += 1;
        }
        return self.i;
    }

    fn trim_right(&mut self) -> usize {
        while self.i < self.j && self.path[self.j - 1] == b'/' {
            self.j -= 1;
        }
        return self.j;
    }

    // returns the unconsumed part of the path
    // eats slashes
    // "/a/b/c" with 1 .next() -> "b/c"
    // "/a/b/c" with 1 .next_back() -> "/a/b"
    // "/" with 1 .next_back() -> ""
    pub fn as_path(&self) -> &'a Path {
        unsafe { Path::from_u8_slice(&self.path[self.i..self.j]) }
    }

    // FIXME other component stuff
}

impl<'a> Iterator for Components<'a> {
    type Item = &'a Path;
    
    fn next(&mut self) -> Option<&'a Path> {
        let start = self.trim_left();
        while self.i < self.j && self.path[self.i] != b'/' {
            self.i += 1;
        }
        let end = self.i;
        self.trim_left();
        if start == end {
            None
        } else {
            Some(unsafe { Path::from_u8_slice(&self.path[start..end]) })
        }
    }
}

impl<'a> DoubleEndedIterator for Components<'a> {
    fn next_back(&mut self) -> Option<&'a Path> {
        let end = self.trim_right();
        while self.i < self.j && self.path[self.j - 1] != b'/' {
            self.j -= 1;
        }
        let start = self.j;
        self.trim_right();
        if start == end {
            None
        } else {
            Some(unsafe { Path::from_u8_slice(&self.path[start..end]) })
        }
    }
}

impl PathBuf {
    fn as_mut_vec(&mut self) -> &mut Vec<u8> {
        unsafe { &mut *(self as *mut PathBuf as *mut Vec<u8>) }
    }
    
    pub fn new() -> PathBuf {
        PathBuf { inner: String::new() }
    }

    pub fn as_path(&self) -> &Path {
        self
    }

    // FIXME all following methods
    pub fn push<P: AsRef<Path>>(&mut self, path: P) {
        if let Some(&b'/') = self.as_mut_vec().last() {
            // last path ends with seperator
        } else if let Some(&b'/') = path.as_ref().as_u8_slice().first() {
            // next path starts with seperator
        } else {
            // add a seperator
            self.inner.push('/');
        }
        self.inner.push_str(path.as_ref().as_ref());
    }
}

impl Path {
    unsafe fn from_u8_slice(s: &[u8]) -> &Path {
        Path::new(mem::transmute::<_, &str>(s))
    }
    
    fn as_u8_slice(&self) -> &[u8] {
        unsafe { mem::transmute(&self.inner) }
    }
    
    pub fn new<S: AsRef<str> + ?Sized>(s: &S) -> &Path {
        unsafe {
            mem::transmute(s.as_ref())
        }
    }

    pub fn as_str(&self) -> &str {
        &self.inner
    }

    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(self.inner.to_string())
    }

    pub fn parent(&self) -> Option<&Path> {
        let mut comps = self.components();
        if let Some(_) = comps.next_back() {
            Some(comps.as_path())
        } else {
            None
        }
    }

    pub fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        let mut owned = self.to_owned();
        owned.push(path);
        owned
    }

    pub fn file_name(&self) -> Option<&str> {
        self.components().next_back().map(|p| p.as_ref())
    }

    pub fn extension(&self) -> Option<&str> {
        self.file_name().and_then(|fname| {
            let mut s = fname.rsplit('.');
            let ext = s.next();
            if s.next().is_some() {
                ext
            } else {
                None
            }

        })
    }

    pub fn components(&self) -> Components {
        Components { path: self.as_u8_slice(), i: 0, j: self.inner.len() }
    }
}

impl<'a, T: ?Sized + AsRef<str>> From<&'a T> for PathBuf {
    fn from(s: &'a T) -> PathBuf {
        PathBuf::from(s.as_ref().to_string())
    }
}

impl From<String> for PathBuf {
    fn from(s: String) -> PathBuf {
        PathBuf { inner: s }
    }
}

// FromIterator, Extend

impl fmt::Debug for PathBuf {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(&**self, formatter)
    }
}

impl Deref for PathBuf {
    type Target = Path;
    fn deref(&self) -> &Path {
        Path::new(&self.inner)
    }
}

impl Borrow<Path> for PathBuf {
    fn borrow(&self) -> &Path {
        self.deref()
    }
}

impl<'a> From<&'a Path> for Cow<'a, Path> {
    #[inline]
    fn from(s: &'a Path) -> Cow<'a, Path> {
        Cow::Borrowed(s)
    }
}

impl<'a> From<PathBuf> for Cow<'a, Path> {
    #[inline]
    fn from(s: PathBuf) -> Cow<'a, Path> {
        Cow::Owned(s)
    }
}

impl ToOwned for Path {
    type Owned = PathBuf;
    fn to_owned(&self) -> PathBuf {
        self.to_path_buf()
    }
}

// by components:
// cmp::PartialEq
// Hash
// cmp::PartialOrd
// cmp::Ord

impl AsRef<str> for PathBuf {
    fn as_ref(&self) -> &str {
        &self.inner[..]
    }
}

impl Into<String> for PathBuf {
    fn into(self) -> String {
        self.inner
    }
}

impl fmt::Debug for Path {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.inner.fmt(formatter)
    }
}

// by components:
// cmp::PartialEq
// Hash
// cmp::Eq
// cmp::PartialOrd
// cmp::Ord

impl AsRef<str> for Path {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl AsRef<Path> for Path {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl AsRef<Path> for str {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<Path> for String {
    fn as_ref(&self) -> &Path {
        Path::new(self)
    }
}

impl AsRef<Path> for PathBuf {
    fn as_ref(&self) -> &Path {
        Path::new(self.inner.as_ref() as &str)
    }
}

impl<'a> IntoIterator for &'a PathBuf {
    type Item = &'a Path;
    type IntoIter = Components<'a>;
    fn into_iter(self) -> Components<'a> {
        self.components()
    }
}

impl<'a> IntoIterator for &'a Path {
    type Item = &'a Path;
    type IntoIter = Components<'a>;
    fn into_iter(self) -> Components<'a> {
        self.components()
    }
}

// partialeq for
// PathBuf, Path
// PathBuf, &'a Path
// Cow<'a, Path>, Path
// Cow<'a, Path>, &'b Path
// Cow<'a, Path>, PathBuf

#[cfg(test)]
mod test {
    use super::*;

    fn comps_as_path(path: &str, i: usize, j: usize, remaining: &str) {
        let p = Path::new(path);
        let mut c = p.components();
        for _ in 0..i {
            c.next();
        }
        for _ in 0..j {
            c.next_back();
        }
        assert_eq!(c.as_path().as_ref() as &str, remaining);
    }

    #[test]
    fn comps_as_path_both() {
        comps_as_path("/a/b/c", 1, 1, "b");
    }

    #[test]
    fn comps_as_path_root() {
        comps_as_path("/a/b/c", 0, 3, "");
    }

    #[test]
    fn comps_as_path_root_exhaust() {
        comps_as_path("/", 0, 1, "");
    }

    #[test]
    fn comps_as_path_none() {
        comps_as_path("/a/b/c", 1, 2, "");
    }

    #[test]
    fn comps_as_path_a() {
        comps_as_path("/a/b/c", 0, 2, "/a");
    }

    #[test]
    fn comps_as_path_b_c() {
        comps_as_path("/a/b/c", 1, 0, "b/c");
    }

    #[test]
    fn comps_as_path_c() {
        comps_as_path("/a/b/c", 2, 0, "c");
    }
    
    #[test]
    fn components() {
        let c: Vec<&str> = Path::new("/a/b/c").components().map(|p| p.as_ref()).collect();
        assert_eq!(c, vec!["a", "b", "c"]);
    }

    #[test]
    fn parent() {
        assert_eq!(Path::new("/a/b/c").parent().map(|p| p.as_ref()), Some("/a/b"));
        assert_eq!(Path::new("/a/b/").parent().map(|p| p.as_ref()), Some("/a"));
        assert_eq!(Path::new("/a").parent().map(|p| p.as_ref()), Some(""));
        assert!(Path::new("/").parent().is_none());
    }
    
    #[test]
    fn file_name() {
        assert_eq!(Path::new("/a/b/cde").file_name(), Some("cde"));
        assert_eq!(Path::new("/a/b/cde").file_name(), Some("cde"));
        assert_eq!(Path::new("/").file_name(), None);
        assert_eq!(Path::new("").file_name(), None);
    }

    #[test]
    fn path_join() {
        let a = Path::new("/a/b");
        let b = Path::new("/a/b/");
        let c = PathBuf::from("/a/b/c");
        assert_eq!(a.join("c").as_str(), c.as_str());
        assert_eq!(b.join("c").as_str(), c.as_str());
    }

    #[test]
    fn path_extension() {
        assert_eq!(Path::new("/a/b/c.txt").extension(), Some("txt"));
        assert_eq!(Path::new("/a/b/c.txt.png").extension(), Some("png"));
        assert_eq!(Path::new("/a/b/c.").extension(), Some(""));
        assert_eq!(Path::new("/a/b.txt/c").extension(), None);
        assert_eq!(Path::new("/").extension(), None);
    }
}
