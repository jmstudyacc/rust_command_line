// Any code not in main() is moved here

use std::error::Error;
use std::fs;

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
    pub fn new(
        query: String,
        filename: String,
        case_sensitive: bool,
    ) -> Result<Config, &'static str> {
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
    let mut count = 0;
    let contents = fs::read_to_string(config.filename)?;

    // check the value of case_sensitive to determine correct search function
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        // this will print each line returned from the search function - for loop returns each line from search
        count += 1;
        println!("{}", line)
    }

    println!(
        "\nThere are {} lines containing the word '{}'.",
        count, &config.query
    );
    /*this syntax is the idiomatic way to call run for side effects only
    it does not return a value that is needed*/
    Ok(())
}

// first iteration of this function will cause the test for 'one_result' to fail
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // uses the filter adaptor to only keep the lines that match the filter criteria
    // the result is then collected into a new vector via collect() and returned
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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
