extern crate regex;
extern crate glob;
extern crate clap;

mod replacer;
mod matching;
mod operation_mode;
mod interactor;
mod arguments;
mod fs_walker;

pub fn main() {
    let args = arguments::parse();

    println!("args.. {:?}", args.file_patterns);
    let mode = if (args.regex_enabled) {
        operation_mode::OperationMode::new_regex(&args.search_pattern).expect("Invalid regex")
    } else {
        operation_mode::OperationMode::new_raw(&args.search_pattern)
    };
}
