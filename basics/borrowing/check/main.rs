fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;

    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");

    // cannot borrow `a` as mutable because
    // it is also borrowed as immutable
    // println!("b: {b}");
}
