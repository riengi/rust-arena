// Reading and writing file in Rust
use std::fs::*;
use std::io::Write;
use std::path::Path;

// Create file if it doesn't exists
fn create_file(path: &str) {
    let path_exists = Path::new(path).exists();
    if path_exists {
        println!("File {} already exists", path);
        return;
    }

    let res = File::create(path);
    match res {
        Ok(f) => println!("File {:?} created", f),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}

// Read content of file
fn read_file(path: &str) {
    let res = read_to_string(path);
    match res {
        Ok(data) => println!("Data read: {}", data),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}

// Open file and appending some text
fn append_ascii_to_file(path: &str) {
    let mut f = OpenOptions::new()
        .append(true)
        .open(path)
        .expect("Unable to open file");

    let str = b"Appended row \n";
    let res = f.write_all(str);

    match res {
        Err(e) => println!("Error: {}", e.to_string()),
        Ok(()) => println!("{} bytes written", str.len()),
    }
}

fn main() {
    let path = "/tmp/rust-text-file.txt";

    create_file(&path);
    append_ascii_to_file(path);
    read_file(&path);
}
