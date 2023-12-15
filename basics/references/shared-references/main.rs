fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;

    println!("r: {}", *r);

    r = &b;
    println!("r: {}", *r);

    // this function's return type contains a borrowed value,
    // but there is no value for it to be borrowed from
    // println!("x_axis: {}", x_axis(2));
}

fn x_axis(x: i32) -> &(i32, i32) {
    let point = (x, 0);
    return &point;
}
