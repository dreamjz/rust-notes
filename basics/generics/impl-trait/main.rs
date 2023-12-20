// Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}

fn main() {
    let many = add_42_millions(42_i8);
    println!("{many}");

    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");

    let debuggable = pair_of(27);
    println!("debuggable: {debuggable:?}");
}
