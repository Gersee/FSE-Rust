use std::env;

fn main() {
    //Vector to access arguments
    let args: Vec<String> = env::args().collect();

    // Print params to terminal
    println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[..]);
}
