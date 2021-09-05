use clap::{App, Arg};
use regex::Regex;
use std::fs::File;
use std::io::stdin;
use std::io::{prelude::*, BufReader};
use std::process::exit;

// use of a generic function to abstract away the issue of file vs. stdin for input
fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        // 'line' is a String but re.find() takes an &str as an argument
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = App::new("final_grep")
        .version("0.1")
        .about("A utility to find target pattern in a file or stdin.")
        .arg(
            Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .help("The pattern that will be used for the search.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("The source that will be searched.")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    // unwrap() to get the value in value_of("pattern") & crash if no value
    let pattern = args.value_of("pattern").unwrap();

    // unwrap() unwraps a Result, crashing if an error occurs (need to handle errors)
    let re = Regex::new(pattern).unwrap();

    // if value_of("file") has no value then input = "-"
    let input = args.value_of("file").unwrap_or("-");

    if input == "-"
    {
        println!("Please enter source you wish to search via standard input:");
        let stdin = stdin();
        let reader = stdin.lock();
        process_lines(reader, re);
    }
    else
    {
        // unwrap() to get the value stored in input & panic if no value
        let reader = BufReader::new(File::open(input).unwrap());
        process_lines(reader, re);
    }
}
