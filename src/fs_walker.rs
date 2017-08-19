use std::path::PathBuf;

use glob::glob;

use itertools::Itertools;

pub struct FsWalker {
    globs: Vec<String>,
    files: Vec<String>,
}

impl FsWalker {
    pub fn new(globs: Vec<String>, files: Vec<String>) -> FsWalker {
        FsWalker {
            globs: globs,
            files: files,
        }
    }

    pub fn iter(&self) -> FsWalkerIterator {
        FsWalkerIterator::new(self)
    }
}

impl<'a> IntoIterator for &'a FsWalker {
    type Item = PathBuf;
    type IntoIter = FsWalkerIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FsWalkerIterator::new(self)
    }
}

pub struct FsWalkerIterator<'a> {
    it: Box<::std::iter::Iterator<Item = ::std::path::PathBuf> + 'a>,
}


impl<'a> FsWalkerIterator<'a> {
    fn new(fs_walker: &'a FsWalker) -> FsWalkerIterator<'a> {
        let a = &fs_walker.globs;
        let b = &fs_walker.files;
        let file_list = b.iter().map(|s| ::std::path::PathBuf::from(s));

        FsWalkerIterator {
            it: Box::new(
                a.iter()
                    //resolve the glob
                    .filter_map(|s| glob(s).ok())
                    //only take Ok results from glob
                    .flat_map(|paths| paths.flat_map(|p| p.ok()))
                    //chain glob iterator with file list iterator
                    .chain(file_list)
                    //uniquify entries.
                    .unique()
                    //only return entries that point to actual files.
                    .filter(|pb| pb.as_path().is_file()),
            ),
        }

    }
}

impl<'a> Iterator for FsWalkerIterator<'a> {
    type Item = PathBuf;
    fn next(&mut self) -> Option<::std::path::PathBuf> {
        self.it.next()
    }
}

#[cfg(test)]
mod fs_walker_test {
    use std::path::{Path, PathBuf};
    use std::fs::{self, File};
    use super::*;

    enum FsEntity {
        Dir(PathBuf),
        File(PathBuf),
    }

    impl Drop for FsEntity {
        fn drop(&mut self) {
            match self {
                &mut FsEntity::Dir(ref p) => {
                    let _ = fs::remove_dir_all(p);
                }
                _ => {}
            }
        }
    }

    fn pb<P: AsRef<Path>>(p: P) -> PathBuf {
        p.as_ref().to_path_buf()
    }
    fn td<P: AsRef<Path>>(p: P) -> FsEntity {
        fs::create_dir_all(&p).unwrap();
        FsEntity::Dir(pb(p))
    }

    fn tf<P: AsRef<Path>>(p: P) -> FsEntity {
        File::create(&p).unwrap();
        FsEntity::File(pb(p))
    }

    fn assert_filelist_glob(glob: &str, file_list: Vec<String>, expected_files: Vec<&str>) {
        let walker = FsWalker::new(vec![glob.into()], file_list);
        let iter = walker.iter();
        let v: Vec<String> = iter.map(|pb| {
            pb.to_str()
                .map_or(Option::None, |opt_str| Some(opt_str.to_owned()))
        }).filter_map(|opt_str| opt_str)
            .collect();

        assert_eq!(expected_files, v);
    }

    #[test]
    fn testing() {
        let _fs = vec![td("t"), tf("t/a"), tf("t/b"), td("t/t"), tf("t/t/a")];
        //normal case
        assert_filelist_glob("t/*", vec!["t/t/a".into()], vec!["t/a", "t/b", "t/t/a"]);

        //duplicates in list
        assert_filelist_glob(
            "t/*",
            vec!["t/t/a".into(), "t/t/a".into()],
            vec!["t/a", "t/b", "t/t/a"],
        );
    }
}
