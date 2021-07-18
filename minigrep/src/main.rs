// bring module into scope as code uses the args() function - may panic due to invalid Unicode
//use std::error::Error;
extern crate clap;

use clap::{App, Arg};
use std::process;

// imports the library crate that has a public API available to test!
use minigrep::Config;

fn main() {
    /* args() function is nested two modules deep std::env::args, as a result convention states
    determines that the parent module is brought into scope and not just the function. This
    allows for use of other functions from std::env
    */
    //let args: Vec<String> = env::args().collect(); // collect() turns iterator into a vector - collect() requires type annotation

    // let yaml = load_yaml!("cli.yaml");
    // let m = App::from(yaml).get_matches();

    let matches = App::new("minigrep")
        .version("1.0")
        .author("James M. <jmstudyacc@gmail.com>")
        .about("Does awesome things")
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
                .help("Sets if search is case-sensitive or case insensitive")
                .required(false),
        )
        .get_matches();

    let query = String::from(matches.value_of("QUERY").unwrap());
    let file = String::from(matches.value_of("FILE").unwrap());
    let case = matches.is_present("CASE");

    /* config now creates a new instance of the Config struct
    main() needs to handle the Result value return*/
    let config = Config::new(query, file, case).unwrap_or_else(|err| {
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
