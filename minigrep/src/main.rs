// bring module into scope as code uses the args() function - may panic due to invalid Unicode
use std::error::Error;
use std::{env, fs, process};

fn main() {
    /* args() function is nested two modules deep std::env::args, as a result convention states
    determines that the parent module is brought into scope and not just the function. This
    allows for use of other functions from std::env
    */
    let args: Vec<String> = env::args().collect(); // collect() turns iterator into a vector - collect() requires type annotation

    // config now creates a new instance of the Config struct
    // main() needs to handle the Result value return
    let config = Config::new(&args).unwrap_or_else(|err| {
        /* unwrap_or_else() requires you to define custom, non-panic! error handling
         if the return is Ok() it acts like unwrap() otherwise it calls the code in the closure
         an anonymous function defined and passed as an argument to unwrap_or_else()
        */
        println!("Problem passing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for '{}'", config.query);
    println!("In file {}", config.filename);

    // isolating main() so it only provides configuration or error handling
    run(config);
}

// declaring run() to separate logic away from main() - returns a Result with a trait object Box<dyn Error> for error type
// requires std::error::Error be imported - the error here will return a dynamic type that implements Error trait
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // small change that extracts this code from main()
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    // this syntax is the idiomatic wy to call run for side effects only
    // it does not return a value that is needed
    Ok(())
}

// Using a struct helps to convey meaning that the two values are related
// it also makes it easier for others to read the code later
struct Config {
    query: String,
    filename: String,
}

impl Config {
    // creating a method via impl against Config is more idiomatic as we are returning Config struct
    fn new(args: &[String]) -> Result<Config, &'static str> {
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
