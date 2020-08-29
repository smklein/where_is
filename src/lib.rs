/*!
Crate `where_is` provides a wrapper on top of `walkdir` for
quickly finding files within a filesystem.

To use this crate, add `where_is` as a dependency to your project's
`Cargo.toml`:

```toml
  [dependencies]
  where_is = "1"
```
*/

#![deny(missing_docs)]

use std::path::Path;
use walkdir::{DirEntry, Result, WalkDir};

/// A file-finding structure.
///
/// Wraps an underlying [`walkdir::WalkDir`] object, and pairs
/// it with a target string used for filtering.
///
/// [`walkdir::WalkDir`]: https://docs.rs/walkdir/latest/walkdir/struct.WalkDir.html
pub struct Finder {
    walker: WalkDir,
    target: String,
}

impl Finder {
    /// Constructs a new `Finder` object, which walks the directory tree
    /// from `root`, looking for a file which matches `target`.
    pub fn new<P: AsRef<Path>>(root: P, target: &str) -> Self {
        Finder {
            walker: WalkDir::new(root),
            target: target.to_string(),
        }
    }
}

impl IntoIterator for Finder {
    type Item = DirEntry;
    type IntoIter = IteratorFilter<walkdir::IntoIter, Box<dyn FnMut(&DirEntry) -> bool>>;

    fn into_iter(self) -> Self::IntoIter {
        let target = self.target;
        IteratorFilter {
            it: self.walker.into_iter(),
            predicate: Box::new(move |entry: &DirEntry| -> bool {
                match entry.path().file_name() {
                    Some(name) => name.to_string_lossy() == target,
                    None => false,
                }
            }),
        }
    }
}

/// An iterator for recursively finding all instances of a file
/// within a directory hierarchy.
pub struct IteratorFilter<I, P> {
    it: I,
    predicate: P,
}

impl<I, P> Iterator for IteratorFilter<I, P>
where
    I: Iterator<Item = Result<DirEntry>>,
    P: FnMut(&DirEntry) -> bool,
{
    type Item = DirEntry;

    fn next(&mut self) -> Option<DirEntry> {
        loop {
            let dent = self.it.next()?.ok()?;
            if !(self.predicate)(&dent) {
                continue;
            }
            return Some(dent);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn find_missing_file() {
        let tmp_dir = TempDir::new("test_where_is").unwrap();
        std::fs::create_dir_all(tmp_dir.path().join("a/b/c")).unwrap();

        let finder = Finder::new(tmp_dir.path(), "does_not_exist");
        let mut iter = finder.into_iter();

        assert!(iter.next().is_none());
    }

    #[test]
    fn find_non_recursive_file() {
        let tmp_dir = TempDir::new("test_where_is").unwrap();
        std::fs::create_dir_all(tmp_dir.path().join("a/b/c")).unwrap();

        let finder = Finder::new(tmp_dir.path(), "a");
        let mut iter = finder.into_iter();

        assert_eq!(tmp_dir.path().join("a"), iter.next().unwrap().path());
        assert!(iter.next().is_none());
    }

    #[test]
    fn find_repeated_recursive_file() {
        let tmp_dir = TempDir::new("test_where_is").unwrap();
        std::fs::create_dir_all(tmp_dir.path().join("a/b/c/a")).unwrap();

        let finder = Finder::new(tmp_dir.path(), "a");
        let mut iter = finder.into_iter();

        assert_eq!(tmp_dir.path().join("a"), iter.next().unwrap().path());
        assert_eq!(tmp_dir.path().join("a/b/c/a"), iter.next().unwrap().path());
        assert!(iter.next().is_none());
    }
}
