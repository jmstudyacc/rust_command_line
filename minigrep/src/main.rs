// bring module into scope as code uses the args() function - may panic due to invalid Unicode
use std::{env, fs};

fn main() {
    /* args() function is nested two modules deep std::env::args, as a result convention states
    determines that the parent module is brought into scope and not just the function. This
    allows for use of other functions from std::env
    */
    let args: Vec<String> = env::args().collect(); // collect() turns iterator into a vector - collect() requires type annotation

    let config = parse_config(&args);

    println!("Searching for '{}'", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// Using a struct helps to convey meaning that the two values are related
// it also makes it easier for others to read the code later
struct Config {
    query: String,
    filename: String,
}

// function to extract the argument parser with a reference to Vector args passed in
// returns an instance of the Config struct
fn parse_config(args: &[String]) -> Config {
    /* Program name takes up arg[0] and we do not want to include this
    So query is indexed from [1] and filename from [2]

    This function now contains the logic needed to define which argument goes in which variable.
    The result is then passed to main()
    */

    // using clone() is easiest but inefficient way to get the data into the Config instance
    // it is more simple and straightforward - sacrifice of performance is worth it here
    let query = args[1].clone(); // value of the first argument passed - stored
    let filename = args[2].clone(); // value of second argument passed - stored

    Config { query, filename }
}
