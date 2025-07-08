use clap::{command, Arg, ArgMatches};

pub fn cli() -> ArgMatches {

    let matches = command!()
    .author("Jakub Němeček")
    .about("Hex dump utility")
    .arg(
        Arg::new("input")
        .help("Input file")
        .required(true)
    )
    .arg(
        Arg::new("columns")
        .short('c')
        .long("columns")
        .required(false)
        .default_value(&"16")
        .value_parser(clap::value_parser!(usize))
    )
    .get_matches();
    matches

}
