use std::process::{Command, Stdio};
use std::io::{Read, Write};
use common::*;


#[test]
fn simple_replace1() {
    let fs = setup(&[tf("file3")]);

    set_file_content("file3", "baca");

    let output = Command::new(BINARY_PATH)
        .args(&["-s", "a", "-x", "Z", "-Y", "it/file3"])
        .output()
        .expect("123");

    //    assert!(output.status.success());
    assert_eq!("", String::from_utf8_lossy(&output.stderr));
    assert_eq!("", String::from_utf8_lossy(&output.stdout));

    assert_eq!("bZcZ", get_file_content("file3"));
}

#[test]
fn ask_user() {
    let fs = setup(&[tf("file3")]);

    set_file_content("file3", "baca");

    let process = Command::new(BINARY_PATH)
        .args(&["-s", "a", "-x", "Z", "it/file3"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("123");

    let mut s = String::new();
    let mut buf: [u8; 4] = [0, 0, 0, 0];
    let mut stdin = process.stdin.unwrap(); //.write_all("alfa".as_bytes());
    //    process.stdin.unwrap().write_all("beta".as_bytes());
    stdin.write_all("yy".as_bytes());
    let mut v: Vec<u8> = Vec::new();
    let mut stdout = process.stdout.unwrap();
    stdout.read_to_string(&mut s);

    assert_eq!(
        "Should replace:
baca
With:
bZca
Should replace:
bZca
With:
bZcZ
",
        s
    );

    //    assert!(output.status.success());
    //    assert_eq!("", String::from_utf8_lossy(&output.stderr));
    //    assert_eq!("", String::from_utf8_lossy(&output.stdout));
}
