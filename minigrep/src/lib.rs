// Any code not in main() is moved here
// std::env is used to check for environment variables - var() to be used

use std::error::Error;
use std::{env, fs};

/* Using a struct helps to convey meaning that the two values are related
it also makes it easier for others to read the code later
case sensitivity is trigger by T/F - run() will need to check for this */
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // creating a method via impl against Config is more idiomatic as we are returning Config struct
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // include a conditional check to catch basic input error
        if args.len() < 3 {
            // instead of calling panic! the function now returns an Err variant
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // new var for case sensitivity, passed the name for case sensitivity ENV variable
        // env::var returns a Result that is Ok if ENV Var is set or Err if ENV Var not set
        // is_err() checks if Result returns Err - not checking the setting of ENV Var, just checking it has a value
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // returns an Ok() variant if not an error
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// declaring run() to separate logic away from main() - returns a Result with a trait object Box<dyn Error> for error type
// requires std::error::Error be imported - the error here will return a dynamic type that implements Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // small change that extracts this code from main()
    let contents = fs::read_to_string(config.filename)?;

    // check the value of case_sensitive to determine correct search function
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        // this will print each line returned from the search function - for loop returns each line from search
        println!("{}", line)
    }

    /*this syntax is the idiomatic way to call run for side effects only
    it does not return a value that is needed*/
    Ok(())
}

// first iteration of this function will cause the test for 'one_result' to fail
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
    the lifetime established here states that the return value should relate to the
    variable 'contents' NOT the ref to 'query'

    returned vec should contain string slices that reference slices of the argument
    'contents'

    1 - iterate over each line of contents
    2 - check whether it contains the query string
    3 - if matching, add to list of values to return
    4 - if not, do nothing
    5 - return the list of matching results
    */
    // create a mutable vector to store the results
    let mut results = Vec::new();

    for line in contents.lines() {
        // lines returns an iterator and is a built in function
        if line.contains(query) {
            // if the line being iterated over contains the query
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /* the function is similar to the search function - difference is that the 'query' reference
    will be made lowercase.

    'query' will be set to lowercase and stored in a shadowed variable. This changes the type.
    shadowed 'query' will now be a string due to the 'to_lowercase()' function. It creates new
    data as opposed to referencing existing data.
    */
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            // &query is used as 'contains()' is defined to take a String slice
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        /* This test searches for the string 'duct'. The test to be searched
        contains three lines with one containing the string 'duct'.

        Assertion states that the value returned contains only the line we expect.*/
        let query = "duct";
        let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        Pick three.\
        Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        /* This test searches for the string 'duct'. The test to be searched
        contains three lines with one containing the string 'duct'.

        Assertion states that the value returned contains only the line we expect.*/
        let query = "rUst";
        let contents = "\
        Rust:\n\
        safe, fast, productive.\n\
        Pick three.\n\
        Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

// test 1: cargo run to poem.txt
// test 2: set CASE_SENSITIVE env variable to 1
//         $env:CASE_SENSITIVE=1/export CASE_SENSITIVE=1
//         cargo run to poem.txt/ CASE_INSENSITIVE=1 cargo run to poem.txt

// EXTRA: allow command line arguments to work as well
