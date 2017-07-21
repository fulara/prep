use glob::glob;

struct FsWalker {
    globs: Vec<String>,
    files: Vec<String>,
}

impl FsWalker {
    fn new(globs: Vec<String>, files: Vec<String>) -> FsWalker {
        FsWalker {
            globs: globs,
            files: files,
        }
    }

    fn iter(&self) -> FsWalkerIterator {
        FsWalkerIterator { fs_walker: self }
    }
}

impl<'a> IntoIterator for &'a FsWalker {
    type Item = &'a str;
    type IntoIter = FsWalkerIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        FsWalkerIterator { fs_walker: self }
    }
}

struct FsWalkerIterator<'a> {
    fs_walker: &'a FsWalker,

//    glob : glob::
//    gl
}

impl<'a> Iterator for FsWalkerIterator<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        return None;

        let s: Vec<String> = vec![];
        let mut it: ::std::slice::Iter<String> = s.iter();
        it.next();
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
            match (self) {
                &mut FsEntity::Dir(ref p) => {
                    fs::remove_dir_all(p);
                }
                _ => {}
            }
        }
    }

    fn pb<P: AsRef<Path>>(p: P) -> PathBuf {
        p.as_ref().to_path_buf()
    }
    fn td<P: AsRef<Path>>(p: P) -> FsEntity {
        fs::create_dir_all(&p);
        FsEntity::Dir(pb(p))
    }

    fn tf<P: AsRef<Path>>(p: P) -> FsEntity {
        File::create(&p);
        FsEntity::File(pb(p))
    }

    fn assert_filelist_glob(glob: &str, expected_files: Vec<&str>) {
        FsWalker::new(vec![glob.into()], vec![]);
    }

    #[test]
    fn testing() {
        let fs = vec![td("t"), tf("t/a"), tf("t/b")];
        assert_filelist_glob("t/*", vec!["t/a", "t/b"]);

        let a = vec!["pattern1/*", "pattern2/"];
        let b = vec!["cc", "dd"];
        let file_list = b.iter().map(|s| ::std::path::PathBuf::from(s));

        for l in a.iter().filter_map(|s| glob(s).ok()).flat_map(|paths| paths.flat_map(|p| p.ok())).chain(file_list) {
            println!("l {:?}", l);
        }


    }
}
