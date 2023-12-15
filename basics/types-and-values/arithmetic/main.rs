fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn main() {
    println!("result: {}", interproduct(120, 100, 248));
}

fn interproduct_overflow(a: i16, b: i16, c: i16) -> i16 {
    return a * b + b * c + c * a;
}
