mod hardware;
use std::env;

fn main() {
    // read the program from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <filename>");
        std::process::exit(2);
    }
}
