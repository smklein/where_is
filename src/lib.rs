use std::path::{Path, PathBuf};
use walkdir::{DirEntry, FilterEntry, Result, WalkDir};

pub struct Finder {
    walker: WalkDir,
    target: String,
}

impl Finder {
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
                    None => false
                }
            }),
        }
    }
}

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

    fn setup() -> TempDir {
        let tmp_dir = TempDir::new("test_where_is").unwrap();
        std::fs::create_dir_all(tmp_dir.path().join("a/b/c")).unwrap();
        tmp_dir
    }

    #[test]
    fn find_missing_file() {
        let tmp_dir = setup();
        let finder = Finder::new(tmp_dir.path(), "does_not_exist");
        assert!(finder.into_iter().next().is_none());
    }

    #[test]
    fn find_non_recursive_file() {
        let tmp_dir = setup();

        let finder = Finder::new(tmp_dir.path(), "a");
        let mut iter = finder.into_iter();
        assert_eq!(tmp_dir.path().join("a"),
                   iter.next().unwrap().path());
        assert!(iter.next().is_none());
    }

    /*
    #[test]
    fn find_repeated_recursive_file() {
        let test_dir = std::env::var("PWD").unwrap() + "/test";
        let target = "baz.txt";
        let golden = vec![
            test_dir.clone() + "/bar/baz.txt",
            test_dir.clone() + "/bar/blat/baz.txt",
        ];

        let finder = Finder::new(test_dir, target);
        let mut iter = finder.into_iter();
        assert_eq!(golden[0],
                   iter.next().unwrap().unwrap().path().to_str().unwrap());
        assert_eq!(golden[1],
                   iter.next().unwrap().unwrap().path().to_str().unwrap());
        assert!(iter.next().is_none());
    }
    */
}
