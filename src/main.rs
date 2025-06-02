use std::{fs, process};

mod cli;

fn main() {

    let matches = cli::cli();
    let input = matches.get_one::<String>("input").unwrap();
    let columns = matches.get_one::<usize>("columns").unwrap();

    let file = fs::File::open(&input);
    let file = match file {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: {err}");
            process::exit(1);
        },
    };

    hd::dump(&file, columns);

}
