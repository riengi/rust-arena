use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize, Debug)]
struct Person {
    name: String,
    age: u8
}



fn main() {

    // Struct
    let person  = Person {name: "str".to_string(), age: 77};
    println!("Struct: {:?}", person);

    // Serialize Struct -> JSON
    let serialized = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {}", serialized);

    // Deserialize JSON -> Struct
    let deserialized: Person = serde_json::from_str(&serialized).unwrap();
    println!("Deserialized struct: {:?}", deserialized);

}
