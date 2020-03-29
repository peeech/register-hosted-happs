use serde_derive::{Serialize, Deserialize};
use std::io::{stdin, Read};

#[derive(Serialize, Deserialize)]
struct Happs {
    hhaids: Vec<String>
}

fn main() {
    let mut buffer = String::new();
    stdin().read_to_string(&mut buffer).unwrap();

    let happs :Happs = toml::from_str(&buffer).unwrap();

    let happs_json = serde_json::to_string(&happs).unwrap();



}
