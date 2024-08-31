mod model;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: bin <text>");
    }

    let query = &args[1];

    let words = query.split(" ");
}
