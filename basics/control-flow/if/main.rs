fn main() {
    let x = 10;

    if x < 20 {
        println!("small");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }

    // use if as an expression
    let size = if x < 20 { "small" } else { "large" };

    println!("number size: {}", size);
}
