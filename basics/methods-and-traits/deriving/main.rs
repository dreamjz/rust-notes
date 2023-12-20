#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default();

    let mut p2 = p1.clone();
    p2.name = String::from("Alice");

    println!("{:?} vs. {:?}", p1, p2);
}
