fn split_string() {
    let str = "Let's have some fun";
    let iter = str.split(" ");
    for i in iter { println!("{}", i)};
}

fn main() {
    split_string();
}
