fn main() {
    let mut v1 = Vec::new();
    v1.push(42);
    print_vec(&v1);

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(999);
    print_vec(&v2);

    // Canonical marco to initialize a vector with elements
    let mut v3 = vec![0, 0, 1, 2, 3];
    print_vec(&v3);

    // Retain only the even elements.
    v3.retain(|x| x & 1 == 0);
    print_vec(&v3);

    // Remove consecutive duplicates
    v3.dedup();
    println!("{v3:?}");
}

fn print_vec(v: &Vec<i64>) {
    println!("v1: len: {}, cap: {}", v.len(), v.capacity());
}
