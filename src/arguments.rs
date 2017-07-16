use clap::{Arg, App, SubCommand};

pub struct Arguments {
    pub search_pattern: String,
    pub replace_pattern: String,
    pub file_patterns: Vec<String>,
    pub files: Vec<String>,
    pub regex_enabled: bool,
}

pub fn parse() -> Arguments {
    let matches = App::new("prep")
        .version("0.1")
        .author("fulara")
        .about("basically very basic sed with rpeview")
        .arg(
            Arg::with_name("search-pattern")
                .help(
                    "Searched pattern, if you want to use regex specify regex flag.",
                )
                .long("pattern")
                .short("s")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("use-regex")
                .help(
                    "Specify whether you want to interpret pattern as a raw text or regex",
                )
                .long("use-regex")
                .short("r"),
        )
        .arg(
            Arg::with_name("replace-pattern")
                .help("Replace pattern, pattern to be replaced")
                .long("replace-with")
                .short("x")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file-patterns")
                .help("File patterns - accepts glob patterns. ")
                .long("file-pattern")
                .short("f")
                .takes_value(true)
                .multiple(true)
                .required(false),
        )
        .arg(
            Arg::with_name("files")
                .help("Files to process")
                .multiple(true)
                .required(false),
        )
        .get_matches();

    let fp = matches
        .values_of("file-patterns")
        .map(|v| v.collect())
        .unwrap_or(Vec::new());

    Arguments {
        search_pattern: matches
            .value_of("search-pattern")
            .expect("search pattern is required")
            .into(),
        replace_pattern: matches
            .value_of("replace-pattern")
            .expect("replace pattern is required")
            .into(),
        file_patterns: matches
            .values_of("file-patterns")
            .map(|v| v.collect())
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|s| String::from(s))
            .collect(),
        files: matches
            .values_of("files")
            .map(|v| v.collect())
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|s| String::from(s))
            .collect(),
        regex_enabled: matches.is_present("use-regex"),
    }

}