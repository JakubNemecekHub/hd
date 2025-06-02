use clap::{command, Arg, ArgMatches};

pub fn cli() -> ArgMatches {

    let matches = command!()
    .arg(
        Arg::new("input")
        .help("Input file")
        .required(true)
    )
    .get_matches();
    matches

}
