extern crate clap;

use clap::{App, Arg};
use std::process;

// imports the library crate that has a public API available to test!
use minigrep::Config;

fn main() {
    let matches = App::new("minigrep")
        .version("1.0")
        .author("James M. <jmstudyacc@gmail.com>")
        .about(
            "Searches a file for a target query string and prints any line containing the query.",
        )
        .arg(
            Arg::with_name("QUERY")
                .short("q")
                .long("query")
                .help("Provides a Slice to be used as query")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("FILE")
                .short("f")
                .long("file")
                .help("Defines the file that will be searched")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("CASE")
                .short("c")
                .long("case-sensitive")
                .multiple(false)
                .takes_value(false)
                .help("Sets if search is case-sensitive or case insensitive [default=false]")
                .required(false),
        )
        .get_matches();

    /* config now creates a new instance of the Config struct
    main() needs to handle the Result value return*/
    let config = Config::new(
        String::from(matches.value_of("QUERY").unwrap()),
        String::from(matches.value_of("FILE").unwrap()),
        matches.is_present("CASE"),
    )
    .unwrap_or_else(|err| {
        /* unwrap_or_else() requires you to define custom, non-panic! error handling
         if the return is Ok() it acts like unwrap() otherwise it calls the code in the closure
         an anonymous function defined and passed as an argument to unwrap_or_else()

         eprintln! macro prints errors to stderr and not stdout, provided by the standard library.
        */
        eprintln!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    /* if run returns an error we return the error value and exit the code
    we return () if no error so we do not need to unwrap anything - we only care about errors*/
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
        // print the error and exit
    }
}

// cargo run to poem.txt > output.txt
// this will print any errors to stderr - you'll see them on the command line
// successful runs of this will send the output to output.txt
