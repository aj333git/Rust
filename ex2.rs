//program argument
use std::env; 
fn main(){ 
//args() function of the Rust’s standard library
//args() function is use to get the arguments
  for argument in env::args() {
  //display each of the argument in separate line through a loop
  //args() function returns the iterator of the arguments.
    println!("{}", argument); 
  } 
}

/*
$ cargo run this is argument by terminal

target/debug/dn2
this
is
argument
by
terminal
*/
