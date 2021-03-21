use std::env;
mod identity;
use identity::{edit_identity, set_identity};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        edit_identity()
    } else if args.len() == 2{
        set_identity(args[1].as_str().to_string())
    } else {
        println!("Invalid arguments")
    }
}
