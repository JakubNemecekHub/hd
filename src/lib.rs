use std::io::{BufReader, Read};
use std::fs;


pub fn dump(file: &fs::File, columns: &usize) -> () {
    let file_size: usize = file.metadata().unwrap().len().try_into().unwrap();
    let magnitude: usize = ((file_size / columns).ilog10() + 1).try_into().unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0; *columns];
    let mut counter = 0;
    print_header(magnitude, *columns);
    loop {
        match reader.read(&mut buffer) {
            Ok(0) => break, // EOF reached
            Ok(n) => {
                print_line_number(counter, magnitude);
                print_line(&buffer, n);
                counter += 1;
            },
            Err(err) => {
                eprintln!("ERROR: {err}");
                break;
            },
        }
    }
}

fn print_header(magnitude: usize, columns: usize) -> () {
    let indentation = magnitude + 3;
    print!("{: <n$}", "", n=indentation);
    for i in 0..columns {
        print!("{:02} ", i);
    }
    println!();
}

fn print_line_number(number: usize, magnitude: usize) -> () {
    print!("{:0magnitude$} | ", number);
}

fn print_line(buffer: &[u8], size: usize) -> () {
    for i in 0..size {
        print!("{} ", format!("{:02X}", buffer[i]));
    }
    let padding = 3 * (buffer.len() - size);
    print!("{: <n$} | ", "", n=padding);
    for i in 0..size {
        let byte = buffer[i];
        if is_printable(byte) {
            print!("{}", buffer[i] as char);
        } else {
            print!(".");
        }
    }
    println!();
}

fn is_printable(byte: u8) -> bool {
    byte >= 0x20 && byte <= 0x7E
}
