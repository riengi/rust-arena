// Smart pointers
use std::env;
mod box_pointer;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for a in args {
        match a.as_str() {
            "box" => {
                box_pointer::int_box();
            }
            _ => {
                println!("Unknown command {}", a);
                println!("Available commands: box")
            }
        }
    }
}
