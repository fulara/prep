use clap::{App, Arg};

pub struct Arguments {
    pub search_pattern: String,
    pub replace_pattern: String,
    pub file_patterns: Vec<String>,
    pub files: Vec<String>,
    pub regex_enabled: bool,
    pub accept_everything: bool,
    pub colorless: bool,
}

pub fn parse() -> Arguments {
    let matches = App::new("prep")
        .version("0.1")
        .author("fulara")
        .about("basically very basic sed with preview")
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
            Arg::with_name("glob-file-search")
                .help("glob file search")
                .long("glob-file-search")
                .short("g")
                .takes_value(true)
                .multiple(true)
                .required(false),
        )
        .arg(
            Arg::with_name("files")
                .help("Files to process")
                .multiple(true)
        )
        .arg(
            Arg::with_name("accept-everything")
                .help(
                    "Will not prompt user whether to accept, just accepts everything",
                )
                .short("Y")
                .long("accept-everything"),
        )
        .arg(
            Arg::with_name("colorless")
                .help("Will disable colors")
                .short("C")
                .long("colorless"),
        )
        .get_matches();

    let opt_values_to_string_list = |s: Option<::clap::Values>| {
        s.map(|v| v.collect())
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|s| String::from(s))
            .collect()
    };

    Arguments {
        search_pattern: matches
            .value_of("search-pattern")
            .expect("search pattern is required")
            .into(),
        replace_pattern: matches
            .value_of("replace-pattern")
            .expect("replace pattern is required")
            .into(),
        file_patterns: opt_values_to_string_list(matches.values_of("glob-file-search")),
        files: opt_values_to_string_list(matches.values_of("files")),
        regex_enabled: matches.is_present("use-regex"),
        accept_everything: matches.is_present("accept-everything"),
        colorless: matches.is_present("colorless"),
    }
}
