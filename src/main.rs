extern crate regex;
extern crate glob;
extern crate clap;
extern crate libc;

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
use std::fs::{File, rename, remove_file};
use libc::getpid;

pub struct TemporaryPrepFile {
    pub writer: BufWriter<File>,
    filename : String,
}

impl TemporaryPrepFile {
    fn new() -> TemporaryPrepFile {
        let filename = Self::generate_filename();
        let wf = File::create(&filename).expect("Could not create temporary file");
        TemporaryPrepFile { writer: BufWriter::new(wf), filename : filename }
    }

    fn filename(&self) -> &str {
        return &self.filename;
    }

    fn generate_filename() -> String {
        format!("prep_tmp_file_{}", unsafe { getpid() })
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

            let mut did_at_least_one_replacement = false;

            while curr.is_some() {
                let line_end = if let &Some(ref l) = &next { "\n" } else { "" };

                let line = &curr.unwrap().expect("Failed to read out a line?");

                if (matching::is_match(&mode, &line)) {
                    write!(tmp.writer, "{}{}", replacer.replace(&line), line_end);
                    did_at_least_one_replacement = true;
                } else {
                    write!(tmp.writer, "{}{}", line, line_end);
                }

                curr = next;
                next = line_iterator.next();
            }


            if (did_at_least_one_replacement) {
                let _ = tmp.writer.flush();
                let _ = rename(tmp.filename(), file);
            } else {
                let _ = remove_file(tmp.filename());
            }
        }
    }

}
