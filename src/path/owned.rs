use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::{Display, Formatter};

use crate::path::components::{Component, Components};
use crate::path::{Path, SEPARATOR};

/// OwnedPath is the equivalent of PathBuf in the normal std library.
/// It is a path whose inner representation is a [`String`].
/// The path can be modified by adding other paths using the [`OwnedPath::push`] method.
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Debug)]
pub struct OwnedPath {
    inner: String,
}

impl<P: AsRef<Path>> From<P> for OwnedPath {
    fn from(v: P) -> Self {
        let mut s = Self::new();
        s.push(v);
        s
    }
}

impl Borrow<Path> for OwnedPath {
    fn borrow(&self) -> &Path {
        Path::new(&self.inner)
    }
}

impl Default for OwnedPath {
    fn default() -> Self {
        Self::new()
    }
}

impl OwnedPath {
    /// Creates a new, empty owned path.
    pub fn new() -> Self {
        Self {
            inner: String::new(),
        }
    }

    /// Appends another path to the end of this path.
    /// If the other path is absolute, but this path is not empty,
    /// then the path will be appended as a relative path.
    ///
    /// ```rust
    /// use kstd::path::owned::OwnedPath;
    /// let mut path = OwnedPath::from("foo");
    /// path.push("/bar");
    /// assert_eq!(path.to_string(), "foo/bar");
    /// ```
    ///
    /// Pushing a parent directory will remove the last component of the path:
    /// ```rust
    /// use kstd::path::owned::OwnedPath;
    /// let mut path = OwnedPath::from("foo/bar");
    /// path.push("..");
    /// assert_eq!(path.to_string(), "foo");
    /// ```
    ///
    /// Pushing a root dir is a no-op if the path is not empty, otherwise it will
    /// make this path absolute.
    /// ```rust
    /// use kstd::path::owned::OwnedPath;
    /// let mut path = OwnedPath::new();
    /// path.push("/");
    /// path.push("foo");
    /// assert_eq!(path.to_string(), "/foo");
    /// ```
    ///
    /// Pushing a current dir is always a no-op and will not modify the path in any way.
    pub fn push<P: AsRef<Path>>(&mut self, segment: P) {
        let path = segment.as_ref();

        path.components().for_each(|c| {
            if !self.is_empty() && self.inner.chars().last().unwrap() != SEPARATOR {
                // we need to push a separator if the rightmost char is not a separator
                self.inner.push(SEPARATOR);
            }

            match c {
                Component::CurrentDir => { /* do nothing here */ }
                Component::ParentDir => {
                    if let Some(index) = self
                        .inner
                        .trim_end_matches(SEPARATOR) // we may already have pushed a SEPARATOR
                        .rfind(SEPARATOR)
                    {
                        let (left, _) = self.inner.split_at(index);
                        self.inner = left.to_owned();
                    }
                }
                Component::Normal(s) => self.inner.push_str(s),
                Component::RootDir => {
                    if self.is_empty() {
                        self.inner.push(SEPARATOR);
                    }
                }
            }
        });
    }

    /// Converts the path into its components.
    ///
    /// ```rust
    /// use kstd::path::owned::OwnedPath;
    /// let mut my_root = OwnedPath::from("/my/path");
    /// assert_eq!(vec![OwnedPath::from("my"), OwnedPath::from("path")], my_root.into_components());
    /// ```
    pub fn into_components(self) -> Vec<OwnedPath> {
        let mut data: Vec<OwnedPath> = Vec::new();

        self.components().for_each(|c| {
            match c {
                Component::ParentDir => {
                    while let Some(last) = data.last() {
                        if last.inner == "." {
                            data.pop();
                        } else {
                            break;
                        }
                    }
                    data.pop();
                }
                Component::CurrentDir => {}
                Component::RootDir => {}
                Component::Normal(p) => data.push(p.into()),
            };
        });

        data
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn components(&self) -> Components<'_> {
        Path::new(&self.inner).components()
    }
}

impl Display for OwnedPath {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec;

    use super::*;

    #[test]
    fn test_push_trivial() {
        let mut p = OwnedPath::new();
        p.push("hello");
        p.push("world");
        assert_eq!("hello/world", p.to_string());
    }

    #[test]
    fn test_push_separators() {
        let mut p = OwnedPath::new();
        p.push("hello");
        p.push("world/");
        assert_eq!("hello/world", p.to_string());
    }

    #[test]
    fn test_push_absolute() {
        let mut p = OwnedPath::new();
        p.push("/hello");
        p.push("world/");
        assert_eq!("/hello/world", p.to_string());
    }

    #[test]
    fn test_into_components() {
        let mut p = OwnedPath::new();
        p.push("segment1");
        p.push("segment2");
        p.push("segment3");
        p.push("segment4");

        assert_eq!(
            vec![
                OwnedPath::from("segment1"),
                OwnedPath::from("segment2"),
                OwnedPath::from("segment3"),
                OwnedPath::from("segment4"),
            ],
            p.into_components()
        );
    }

    #[test]
    fn test_into_components_with_specials() {
        let mut p = OwnedPath::new();
        p.push("segment1");
        p.push("segment2");
        p.push("..");
        p.push("segment3");
        p.push("segment4");
        p.push(".");
        p.push("..");
        p.push("segment5");

        assert_eq!(
            vec![
                OwnedPath::from("segment1"),
                OwnedPath::from("segment3"),
                OwnedPath::from("segment5"),
            ],
            p.into_components()
        );
    }
}
