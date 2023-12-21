#[derive(Default, Debug)]
struct Backends {
    hostnames: Vec<String>,
    weights: Vec<f64>,
}

impl Backends {
    fn set_hostname(&mut self, hostnames: Vec<String>) {
        self.hostnames = hostnames.clone();
        self.weights = hostnames.iter().map(|_| 1.0).collect();
    }
}

fn main() {
    let mut be = Backends::default();
    println!("{be:?}");

    let hns: Vec<String> = vec!["A".to_string()];
    be.set_hostname(hns);
    println!("{be:?}");
}