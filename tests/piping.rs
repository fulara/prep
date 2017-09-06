use common::*;
use std::process::{Command, Stdio};
use std::io::{Read, Write};


#[test]
fn text_mode() {
    let process = Command::new(BINARY_PATH)
        .args(&["-s", "a", "-x", "b", "-C"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("123");

    let mut output = String::new();
    let mut stdin = process.stdin.unwrap();
    stdin.write_all("xaxaxa".as_bytes()).unwrap();

    drop(stdin);
    let mut stdout = process.stdout.unwrap();
    stdout.read_to_string(&mut output).unwrap();

    assert_eq!("xbxbxb\n", output);
}


#[test]
fn regex_mode_mode() {
    let process = Command::new(BINARY_PATH)
        .args(&["-s", "(a)", "-x", "${1}${1}", "-r", "-C"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("123");

    let mut output = String::new();
    let mut stdin = process.stdin.unwrap();
    stdin.write_all("xaxaxa".as_bytes()).unwrap();

    drop(stdin);
    let mut stdout = process.stdout.unwrap();
    stdout.read_to_string(&mut output).unwrap();

    assert_eq!("xaaxaaxaa\n", output);
}
