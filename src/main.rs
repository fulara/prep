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
use std::fs::rename;

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
        let wf = File::create(Self::generate_file_name()).expect("Could not create temporary file");
        TemporaryPrepFile { writer: BufWriter::new(wf) }
    }

    fn file_name(&self) -> &'static str {
        return "prep_tmp_file";
    }

    fn generate_file_name() -> String {
        "prep_tmp_file".into()
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
        let f = File::open(file.clone());
        if let Ok(f) = f {
            let f = BufReader::new(f);
            let mut tmp = TemporaryPrepFile::new();

            let mut line_iterator = f.lines();
            let mut curr = line_iterator.next();
            let mut next = line_iterator.next();

            while curr.is_some() {
                let line_end = if let &Some(ref l) = &next { "\n" } else { "" };

                let line = &curr.unwrap().expect("Failed to read out a line?");

                if (matching::is_match(&mode, &line)) {
                    write!(tmp.writer, "{}{}", replacer.replace(&line), line_end);
                } else {
                    write!(tmp.writer, "{}{}", line, line_end);
                }

                curr = next;
                next = line_iterator.next();
            }


            tmp.writer.flush();
            rename(tmp.file_name(), file);
        }
    }

}
