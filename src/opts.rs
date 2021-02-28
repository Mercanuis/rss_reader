use clap::{App, Arg, ArgMatches};

const APP_NAME: &str = "rss_reader";
const INPUT: &str = "input";
const OUTPUT: &str = "output";
const URL: &str = "url";
const REPLACE: &str = "replace";
const TRIM: &str = "trim";

#[derive(Debug)]
pub struct Args {
    matches: ArgMatches,
}

impl Args {
    pub fn new() -> Self {
        Args {
            matches: get_matches(),
        }
    }

    pub fn get_input(&self) -> Option<&str> {
        self.matches.value_of(INPUT)
    }

    pub fn get_url(&self) -> Option<&str> {
        self.matches.value_of(URL)
    }

    pub fn get_output(&self) -> Option<&str> {
        self.matches.value_of(OUTPUT)
    }

    pub fn get_replace(&self) -> Option<&str> {
        self.matches.value_of(REPLACE)
    }

    pub fn is_trim(&self) -> bool {
        self.matches.is_present(TRIM)
    }
}

fn get_matches() -> ArgMatches {
    App::new(APP_NAME)
        .version("1.0")
        .author("Joseph Orme <brogramn@gmail.com")
        .about("RSS Reader")
        .arg(
            Arg::new(INPUT)
                .short('i')
                .long(INPUT)
                .takes_value(true)
                .value_name("FILE")
                .about("Sets a file for input reading (CSV expected, any other extension will throw an error")
                .conflicts_with("url")
        )
        .arg(
            Arg::new(URL)
                .short('u')
                .long(URL)
                .takes_value(true)
                .value_name("URL")
                .about("Sets a URL to use for input reading (for an RSS stream)")
                .conflicts_with("input")
        )
        .arg(
            Arg::new(OUTPUT)
                .short('o')
                .long(OUTPUT)
                .takes_value(true)
                .value_name("OUT")
                .about("specifies output file OUT, optional, default output will write to console")
        )
        .arg(
            Arg::new(TRIM)
                .short('t')
                .long(TRIM)
                .about("trims the feed of lines, to 10 characters should any exceed")
        )
        .arg(
            Arg::new(REPLACE)
                .short('r')
                .long(REPLACE)
                .takes_value(true)
                .value_name("REPLACE")
                .about("FROM|TO replaces the 'from' with the 'to'")
        )
        .get_matches()
}
