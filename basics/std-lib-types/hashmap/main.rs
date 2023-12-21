use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert("A", 1);
    m.insert("B", 2);
    m.insert("C", 3);
    println!("m: {m:?}");

    println!("m contains D: {}", m.contains_key("D"));
    println!("m contains A: {}", m.contains_key("A"));

    match m.get("A") {
        Some(v) => println!("A: {}", v),
        None => println!("A is not exists"),
    }

    // insert a entry with default val when the key is not found
    let e = m.entry("D").or_insert(0);
    *e += 1;
    println!("{e:#?}");
    
}