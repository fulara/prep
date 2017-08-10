extern crate regex;
extern crate glob;
extern crate clap;

#[macro_use]
extern crate itertools;

mod replacer;
mod matching;
mod operation_mode;
mod interactor;
mod arguments;
mod fs_walker;


use std::io::{self, BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;

//let f = File::open("foo.txt")?;
//let f = BufReader::new(f);
//
//for line in f.lines() {
//println!("{}", line.unwrap());
//}

pub struct TemporaryPrepFile {
    pub writer: BufWriter<File>,
}

impl TemporaryPrepFile {
    fn new() -> TemporaryPrepFile {
        let wf = File::create("prep_tmp_file").expect("Could not create temporary file");
        TemporaryPrepFile { writer: BufWriter::new(wf) }
    }
}

impl Drop for TemporaryPrepFile {
    fn drop(&mut self) {
//        drop(self.writer);
    }
}

pub fn main() {
    let args = arguments::parse();

    let mode = if (args.regex_enabled) {
        operation_mode::OperationMode::new_regex(&args.search_pattern).expect("Invalid regex")
    } else {
        operation_mode::OperationMode::new_raw(&args.search_pattern)
    };

    let replacer = replacer::Replacer::new(mode.clone(), &args.replace_pattern);

    let walker = fs_walker::FsWalker::new(args.file_patterns, args.files);
    for file in walker.iter() {
        let f = File::open(file);
        if let Ok(f) = f {
            let f = BufReader::new(f);
            let mut tmp = TemporaryPrepFile::new();

            for line in f.lines() {
                let line = &line.expect("Failed during reading file line by line...");
                if (matching::is_match(&mode, line)) {
                    writeln!(tmp.writer, "{}", replacer.replace(line));
                } else {
                    writeln!(tmp.writer, "{}", line);
                }
            }
        }
    }

}
