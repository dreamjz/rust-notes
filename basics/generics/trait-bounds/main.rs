fn duplicate<T>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);

    println!("{pair:?}");
}
