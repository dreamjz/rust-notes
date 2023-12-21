#[derive(Debug)]
enum List<T> {
    /// A non-empty list, consisting of the first element and the rest of the list.
    Element(T, Box<List<T>>),
    /// An empty list.
    Nil,
}

fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);

    let list: List<i32> = List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");
}
