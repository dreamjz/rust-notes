struct Dog {
    name: String,
    age: i8,
}

struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("What's your name? {}", self.talk());
    }
}

impl Pet for Dog {
    fn talk(&self) -> String {
        return format!("Woof, my name is {}", self.name);
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        return String::from("WTF?");
    }
}

fn main() {
    let c = Cat { lives: 9 };
    let d = Dog {
        name: String::from("Fido"),
        age: 5,
    };

    c.greet();
    d.greet();
}
