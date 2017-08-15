use std::process::Command;

use common::*;


#[test]
fn simple_replace1() {
    let fs = setup(&[tf("file3")]);

    set_file_content("file3", "baca");

    let output = Command::new(BINARY_PATH).args(&["-s", "a", "-x", "Z", "it/file3"]).output().expect("123");

    assert!(output.status.success());
    assert_eq!("", String::from_utf8_lossy(&output.stderr));
    assert_eq!("", String::from_utf8_lossy(&output.stdout));

    assert_eq!("bZbZ", get_file_content("file3"));
}