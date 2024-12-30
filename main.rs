use std::env;
  
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("this is all you got  {:?} .", args.len()-1);
}
