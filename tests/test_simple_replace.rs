use std::process::{Command, Stdio};
use std::io::{Read, Write};
use common::*;


#[test]
fn simple_replace1() {
    let _fs = setup(&[tf("file3")]);

    set_file_content("file3", "baca");

    let output = Command::new(BINARY_PATH)
        .args(&["-s", "a", "-x", "Z", "-Y", "-Q", "it/file3"])
        .output()
        .expect("123");

    assert!(output.status.success());
    assert_eq!("", String::from_utf8_lossy(&output.stderr));
    assert_eq!("", String::from_utf8_lossy(&output.stdout));

    assert_eq!("bZcZ", get_file_content("file3"));
}

#[test]
fn ask_user_user_accepts_all() {
    let _fs = setup(&[tf("file3")]);

    set_file_content("file3", "baca");

    let process = Command::new(BINARY_PATH)
        .args(&["-s", "a", "-x", "Z", "-C", "-Q", "it/file3"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("123");

    let mut output = String::new();
    let mut stdin = process.stdin.unwrap();
    stdin.write_all("yy".as_bytes()).unwrap();
    let mut stdout = process.stdout.unwrap();
    stdout.read_to_string(&mut output).unwrap();

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
        output
    );

    assert_eq!("bZcZ", get_file_content("file3"))
}


#[test]
fn ask_user_user_rejects_some() {
    let _fs = setup(&[tf("file3")]);

    set_file_content("file3", "babababa");

    let process = Command::new(BINARY_PATH)
        .args(&["-s", "a", "-x", "Z", "-C", "-Q", "it/file3"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("123");

    let mut output = String::new();
    let mut stdin = process.stdin.unwrap(); //.write_all("alfa".as_bytes());
    //    process.stdin.unwrap().write_all("beta".as_bytes());
    stdin.write_all("ynyn".as_bytes()).unwrap();
    let mut stdout = process.stdout.unwrap();
    stdout.read_to_string(&mut output).unwrap();

    assert_eq!(
        "Should replace:
babababa
With:
bZbababa
Should replace:
bZbababa
With:
bZbZbaba
Should replace:
bZbababa
With:
bZbabZba
Should replace:
bZbabZba
With:
bZbabZbZ
",
        output
    );

    assert_eq!("bZbabZba", get_file_content("file3"));
}
