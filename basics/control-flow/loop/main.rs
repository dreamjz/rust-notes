fn main() {
    let mut i = 0;

    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }
}
