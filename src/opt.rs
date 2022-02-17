use clap::{app_from_crate, arg};

pub fn parse_opt() -> clap::ArgMatches {
    app_from_crate!()
        .arg(arg!(<PART>))
        .get_matches()
}
