fn main() {
    let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];

    // cannot assign to `a[_]` because it is borrowed
    // a[3] = 400;

    println!("s: {s:?}");

    a[3] = 400;
    println!("a: {a:?}");
}
