use clap::{App, Arg};
use regex::Regex;

fn main() {
    // Incrementally builds a command argument parser, where each argument takes an Arg
    let args = App::new("grep_regex")
        .version("0.1")
        .about("Searches for patterns.")
        .arg(
            Arg::with_name("pattern")
                .short("p")
                .long("pattern")
                .help("The pattern that will be used for the search.")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    // extracts the pattern argument from the args parser
    let pattern = args.value_of("pattern").unwrap();

    // unwrap() unwraps a Result, crashing if an error occurs (need to handle errors)
    let re = Regex::new(pattern).unwrap();

    let source = "\
    Lorem ipsum dolor sit amet, consectetur adipiscing elit,\n\
    sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\n\
    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.\n\
    Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur.\n\
    Excepteur sint occaecat cupidatat non proident,\n\
    sunt in culpa qui officia deserunt mollit anim id est laborum.";

    for line in source.lines() {
        // finds the left-most match, in target, of the value being called against
        let contains_substring = re.find(line);

        // replaces the .contains() approach with a match block - requires handling of all possible cases
        match contains_substring {
            // Some(T) is positive case, here we are saying if Any positive case, print the line
            Some(_) => println!("{}", line),
            // None is negative case of Option() - here it is saying if no matches then return nothing
            None => (),
        }
    }
}
