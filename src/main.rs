use std::{fs, process};

mod cli;

fn main() {

    let matches = cli::cli();
    let input = matches.get_one::<String>("input").unwrap();

    let file = fs::File::open(&input);
    let file = match file {
        Ok(file) => file,
        Err(err) => {
            eprintln!("ERROR: {err}");
            process::exit(1);
        },
    };

    const BUFFER_SIZE: usize = 16;
    hd::dump(&file, BUFFER_SIZE);

}
