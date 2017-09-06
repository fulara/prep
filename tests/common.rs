use std::io::Read;
use std::io::Write;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::path::PathBuf;
use std::path::Path;

const IT_PATH: &str = "it";
pub const BINARY_PATH: &str = "target/debug/prep";

pub fn get_file_content(file: &str) -> String {
    let path_buf = pb(file);
    let mut file = File::open(path_buf.clone()).expect(&format!("opening file {:?}", path_buf));
    let mut text = String::new();
    file.read_to_string(&mut text).expect("reading file");

    text
}

pub fn set_file_content(file: &str, content: &str) {
    let path_buf = pb(file);
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path_buf)
        .expect("set_file_content opening file");
    file.write_all(content.as_bytes()).unwrap();
}

#[derive(Debug)]
pub enum FsEntity {
    Dir(PathBuf),
    File(PathBuf),
}

pub struct TestFs {}

impl Drop for TestFs {
    fn drop(&mut self) {
        fs::remove_dir_all(IT_PATH).unwrap();
    }
}

impl TestFs {
    fn create(entities: &[FsEntity]) -> TestFs {
        fs::create_dir_all(IT_PATH).expect("Failed during creation of it directory");

        for entity in entities {
            match entity {
                &FsEntity::Dir(ref pb) => {
                    fs::create_dir_all(pb).unwrap();
                }
                &FsEntity::File(ref pb) => {
                    File::create(pb).unwrap();
                }
            }
        }

        TestFs {}
    }
}

pub fn pb<P: AsRef<Path>>(p: P) -> PathBuf {
    let mut pb = PathBuf::from("it/");
    pb.push(p);
    pb
}
pub fn td<P: AsRef<Path>>(p: P) -> FsEntity {
    FsEntity::Dir(pb(p))
}

pub fn tf<P: AsRef<Path>>(p: P) -> FsEntity {
    FsEntity::File(pb(p))
}

pub fn setup(entities: &[FsEntity]) -> TestFs {
    TestFs::create(&entities)
}
