extern crate prep;
extern crate glob;

use glob::glob;

#[test]
fn foo() {
    for entry in glob("*target*/*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}