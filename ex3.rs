//arg 3
use std::env;

fn main() {
//env::args().collect(); returns the argument passed in the terminal.
//also  shows the number of arguments.
    let args: Vec<String> = env::args().collect();

    // The first argument is the path that was used to call the program.
    println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    
    
    
}


/*
$ cargo run two arguments
My path is target/debug/dn2.
I got 2 arguments: ["two", "arguments"].
*/
