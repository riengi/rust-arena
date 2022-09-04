// Collections
use std::env;
mod tree;
mod vector;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for a in args {
        match a.as_str() {
            "vector" => {
                vector::basic_vector();
                vector::various_types_vector()
            }
            "tree" => {
                tree::list();
            }
            _ => {
                println!("Unknown command {}", a);
                println!("Available commands: vector, tree")
            }
        }
    }
}
