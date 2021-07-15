// bring module into scope as code uses the args() function - may panic due to invalid Unicode
use std::{env, fs};

fn main() {
    /* args() function is nested two modules deep std::env::args, as a result convention states
    determines that the parent module is brought into scope and not just the function. This
    allows for use of other functions from std::env
    */
    let args: Vec<String> = env::args().collect(); // collect() turns iterator into a vector - collect() requires type annotation

    // config now creates a new instance of the Config struct
    let config = Config::new(&args);

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

impl Config {
    // creating a method via impl against Config is more idiomatic as we are returning Config struct
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
