// Any code not in main() is moved here

use std::error::Error;
use std::fs;

// Using a struct helps to convey meaning that the two values are related
// it also makes it easier for others to read the code later
pub struct Config {
    pub query: String,
    pub filename: String,
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

        // returns an Ok() variant if not an error
        Ok(Config { query, filename })
    }
}

// declaring run() to separate logic away from main() - returns a Result with a trait object Box<dyn Error> for error type
// requires std::error::Error be imported - the error here will return a dynamic type that implements Error trait
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // small change that extracts this code from main()
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    // this syntax is the idiomatic wy to call run for side effects only
    // it does not return a value that is needed
    Ok(())
}
