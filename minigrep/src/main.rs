// bring module into scope as code uses the args() function - may panic due to invalid Unicode
//use std::error::Error;
extern crate clap;

use clap::{load_yaml, App};
use std::{env, process};

// imports the library crate that has a public API available to test!
use minigrep::Config;

fn main() {
    /* args() function is nested two modules deep std::env::args, as a result convention states
    determines that the parent module is brought into scope and not just the function. This
    allows for use of other functions from std::env
    */
    let args: Vec<String> = env::args().collect(); // collect() turns iterator into a vector - collect() requires type annotation

    /* config now creates a new instance of the Config struct
    main() needs to handle the Result value return*/
    let config = Config::new(&args).unwrap_or_else(|err| {
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
