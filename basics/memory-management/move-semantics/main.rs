fn main() {
    let s1: String = String::from("Hello");
    let s2: String = s1;
    println!("s2: {s2}");

    // borrow of moved value: `s1`
    // println!("s1: {s1}");

    let name = String::from("Alice");
    say_hello(name);

    // use of moved value: `name`
    // say_hello(name);

    // pass a clone of name
    let name2 = String::from("Alice");
    say_hello(name2.clone());
    say_hello(name2.clone());

    // main func can retain ownership by passing name as reference    
    let name3 = &String::from("Kesa");
    say_hello_ref(name3);
    say_hello_ref(name3);
}

fn say_hello(name: String) {
    println!("Hello {name}");
}

fn say_hello_ref(name: &String) {
    println!("Hello {name}");
}
