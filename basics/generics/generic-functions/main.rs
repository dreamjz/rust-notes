fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n & 1 == 0 {
        even
    } else {
        odd
    }
}

fn main() {
    println!("picked a number: {:?}", pick(97, 22, 33));
    println!("picked a tuple: {:?}", pick(28, ("dog", 1), ("cat", 2)));
}
