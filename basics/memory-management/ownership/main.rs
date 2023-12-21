struct Point (i32, i32);

fn main() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }

    // cannot find value `p` in this scope
    // println!("y: {}", p.1);
}