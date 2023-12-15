fn main() {
    let x:i32 = 10;
    println!("x: {x}");
    
    // error[E0384]: cannot assign twice to immutable variable `x`
    // x = 20;  
    // println!("x: {x}");

    // use mut keyword to allow changes
    let mut y: i64 = 100;
    println!("y: {y}");
    
    y = 200;
    println!("y: {y}");
}