use crate::path::{Path, SEPARATOR};
use alloc::vec::Vec;

pub struct Components<'a> {
    fragments: Vec<&'a str>,
    seen_root: bool,
    absolute: bool,
    index: usize,
}

impl<'a> Components<'a> {
    pub fn new(path: &'a Path) -> Self {
        let absolute = path.starts_with(SEPARATOR);
        Components {
            fragments: path.split(SEPARATOR).collect(),
            seen_root: false,
            absolute,
            index: 0,
        }
    }
}

/// Paths are made of single components. If a path is absolute,
/// the first component is the [`Component::RootDir`]. Every subsequent
/// component that is not a `.` or a `..` is a [`Component::Normal`].
/// Empty segments in a path will be ignored.
///
/// ## Examples
/// `/foo/./bar` produces the following iterator:
/// ```rust
/// use kstd::path::components::Component;
/// use kstd::path::Path;
/// let path = Path::new("/foo/./bar");
/// let mut components = path.components();
/// assert_eq!(components.next(), Some(Component::RootDir));
/// assert_eq!(components.next(), Some(Component::Normal("foo")));
/// assert_eq!(components.next(), Some(Component::CurrentDir));
/// assert_eq!(components.next(), Some(Component::Normal("bar")));
/// assert_eq!(components.next(), None);
/// ```
///
/// `foo///bar` produces the following iterator:
/// ```rust
/// use kstd::path::components::Component;
/// use kstd::path::Path;
/// let path = Path::new("foo///bar");
/// let mut components = path.components();
/// assert_eq!(components.next(), Some(Component::Normal("foo")));  
/// assert_eq!(components.next(), Some(Component::Normal("bar")));
/// assert_eq!(components.next(), None);
/// ```
///
/// `//foo/bar` produces the following iterator:
/// ```rust
/// use kstd::path::components::Component;
/// use kstd::path::Path;
/// let path = Path::new("//foo/bar");
/// let mut components = path.components();
/// assert_eq!(components.next(), Some(Component::RootDir));
/// assert_eq!(components.next(), Some(Component::Normal("foo")));
/// assert_eq!(components.next(), Some(Component::Normal("bar")));
/// assert_eq!(components.next(), None);
/// ```
#[derive(Debug, Eq, PartialEq)]
pub enum Component<'a> {
    /// The root dir component. If it is encountered, it is always
    /// the first component of a path, and the path is absolute.
    /// If this is not encountered, then the path is not absolute.
    RootDir,
    /// The current directory, produced if the path contains a `.`
    /// as a part, such as in `/foo/./bar`.
    CurrentDir,
    /// The parent directory, produced if the path contains a `..`
    /// as a part, such as in `/foo/../bar`.
    ParentDir,
    /// A normal component, which is a non-empty string. Empty
    /// parts in the path are ignored, meaning that a path like
    /// `foo//bar` and `foo///bar` will have two components,
    /// `foo` and `bar`.
    Normal(&'a str),
}

impl<'a> Iterator for Components<'a> {
    type Item = Component<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.absolute && !self.seen_root {
            self.seen_root = true;
            return Some(Component::RootDir);
        }

        let mut item = "";
        while item.is_empty() {
            if self.index >= self.fragments.len() {
                return None;
            }

            // hello///world should be equivalent to hello/world, so we skip empty fragments
            item = self.fragments[self.index];
            self.index += 1;
        }

        match item {
            "." => Some(Component::CurrentDir),
            ".." => Some(Component::ParentDir),
            _ => Some(Component::Normal(item)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_components() {
        let p = Path::new("hello/world");
        let mut c = p.components();
        assert_eq!(Some(Component::Normal("hello")), c.next());
        assert_eq!(Some(Component::Normal("world")), c.next());
        assert_eq!(None, c.next());
    }

    #[test]
    fn test_components_absolute() {
        let p = Path::new("/hello/world");
        let mut c = p.components();
        assert_eq!(Some(Component::RootDir), c.next());
        assert_eq!(Some(Component::Normal("hello")), c.next());
        assert_eq!(Some(Component::Normal("world")), c.next());
        assert_eq!(None, c.next());
    }

    #[test]
    fn test_empty_fragments() {
        let p = Path::new("hello///world");
        let mut c = p.components();
        assert_eq!(Some(Component::Normal("hello")), c.next());
        assert_eq!(Some(Component::Normal("world")), c.next());
        assert_eq!(None, c.next());
    }
}
