use std::fs::File;
use std::io::{BufReader, prelude::*};

fn main() {
    // creates a File object that requires a path argument
    // at present this program will crash if "README.md" does not exist
    let f = File::open("README.md").unwrap();

    /*
        // reader == new BufReader struct created using 'f' as the source
        let mut reader = BufReader::new(f);

            // reuses the 'line' String object over the lifetime of the program - String object is mutable
            let mut line = String::new();

            loop
            {
                // reading from disk can fail so the error needs to be handled, lazily in this case with unwrap()
                let len = reader.read_line(&mut line).unwrap();

                if len == 0
                {
                    break
                }

                println!("{} ({} bytes long)", line, len);

                // shrinks the String back to length 0 - preventing lines from persisting into following lines
                line.truncate(0);
            }*/
    /*
    manually looping through a file is unwieldy and
    Rust provides an iterator to iterate through lines with BufReader::lines().
    BufReader::lines() also removes the trailing newline character from each line
    */
    // clears the compiler warning as reader no longer needs to be mutable
    let reader = BufReader::new(f);

    for line_ in reader.lines()
    {
        // unwraps the Result but at the risk of crashing the program if an error occurs
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
