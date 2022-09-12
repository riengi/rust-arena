// Hashmap in Rust

use std::collections::HashMap;

pub fn hashmap() {
    let mut hm = HashMap::new();
    hm.insert("1", 1);

    println!("{}", hm.get("1").unwrap());

    // Update hashmap value
    // this doesn't work due to missing IndexMut trait
    // hm["1"] = 2;

    // workaround
    *hm.get_mut("1").unwrap() += 1;
    println!("{}", hm.get("1").unwrap());
}
