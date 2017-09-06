extern crate clap;
extern crate colored;
extern crate glob;
extern crate libc;
extern crate regex;
//#[macro_use]

extern crate itertools;

mod replacer;
mod operation_mode;
mod interactor;
mod arguments;
mod fs_walker;


use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::{remove_file, rename, File};
use libc::getpid;

use colored::*;

use interactor::{ask_user, InteractionResult};

pub struct TemporaryPrepFile {
    pub writer: BufWriter<File>,
    filename: String,
}

impl TemporaryPrepFile {
    fn new() -> TemporaryPrepFile {
        let filename = Self::generate_filename();
        let wf = File::create(&filename).expect("Could not create temporary file");
        TemporaryPrepFile {
            writer: BufWriter::new(wf),
            filename: filename,
        }
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

    let mode = if args.regex_enabled {
        operation_mode::OperationMode::new_regex(&args.search_pattern).expect("Invalid regex")
    } else {
        operation_mode::OperationMode::new_raw(&args.search_pattern)
    };

    colored::control::set_override(!args.colorless);

//    let stdin = ::std::io::stdin();
//    let stdin = stdin.lock();
//
//    for l in stdin.lines() {
//        println!("line1 is: {:?}", l);
//    }
//
//    return;



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
                let line_end = if next.is_some() { "\n" } else { "" };

                let mut line = curr.unwrap().expect("Failed to read out a line?").clone();

                let mut pos = 0usize;

                loop {
                    line = if let Some(result) = replacer.replace(&line, pos) {
                        pos = result.position_of_replacement;
                        let mut should_do_replacement = args.accept_everything;

                        if !should_do_replacement {
                            match ask_user(&format!(
                                "Should replace:\n{}{}{}\nWith:\n{}{}{}",
                                result.before,
                                result.old.green(),
                                result.after,
                                result.before,
                                result.new.red(),
                                result.after
                            )) {
                                InteractionResult::Accept => {
                                    should_do_replacement = true;
                                }
                                _ => {}
                            }
                        }

                        if should_do_replacement {
                            did_at_least_one_replacement = true;
                            format!("{}{}{}", result.before, result.new, result.after)
                        } else {
                            pos = result.before.len() + result.old.len();
                            format!("{}{}{}", result.before, result.old, result.after)
                        }
                    } else {
                        break;
                    }
                }

                write!(tmp.writer, "{}{}", line, line_end)
                    .expect("Failed to write line to temp file.");

                curr = next;
                next = line_iterator.next();
            }


            if did_at_least_one_replacement {
                let _ = tmp.writer.flush();
                let _ = rename(tmp.filename(), file);
            } else {
                let _ = remove_file(tmp.filename());
            }
        }
    }
}
