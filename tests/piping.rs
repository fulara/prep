use common::*;
use std::process::{Command, Stdio};
use std::io::{Read, Write};


#[test]
fn piping() {
    let process = Command::new(BINARY_PATH)
        .args(&["-s", "a", "-x", "Z", "-C"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("123");

    println!("hai");

    let mut output = String::new();
    let mut stdin = process.stdin.unwrap();
    stdin.write_all("replace_me
y".as_bytes()).unwrap();

    drop(stdin);
    println!("hai");
    let mut stdout = process.stdout.unwrap();
    println!("hai2");
    stdout.read_to_string(&mut output).unwrap();

    println!("hai2");

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

//    assert_eq!("bZcZ", get_file_content("file3"))
}
