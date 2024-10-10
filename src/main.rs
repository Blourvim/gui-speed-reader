mod model;
mod view;

use std::env;
use std::io::{self, Write};

use model::DisplayArray;

fn main() {
    // Get the input from the command line
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Usage: <bin> <text>");
    }
    let input = &args[1];
    let words: Vec<&str> = input.split_whitespace().collect();
    let mut display_array = DisplayArray::new();

    for word in words {
        display_array.print();
        display_array.update(word);
    }
        io::stdout().flush().unwrap();
}

