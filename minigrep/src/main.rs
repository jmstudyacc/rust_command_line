// bring module into scope as code uses the args() function - may panic due to invalid Unicode
use std::env;

fn main() {
    /* args() function is nested two modules deep std::env::args, as a result convention states
    determines that the parent module is brought into scope and not just the function. This
    allows for use of other functions from std::env
    */
    let args: Vec<String> = env::args().collect(); // collect() turns iterator into a vector - collect() requires type annotation

    /* Program name takes up arg[0] and we do not want to include this
    So query is indexed from [1] and filename from [2]
    */
    let query = &args[1]; // value of the first argument passed - stored
    let filename = &args[2]; // value of second argument passed - stored

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
